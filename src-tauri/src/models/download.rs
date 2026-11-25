use std::path::Path;
use std::time::Duration;

use reqwest::header::{CONTENT_RANGE, RANGE};
use reqwest::{Client, StatusCode};
use sha2::{Digest, Sha256};
use tokio::fs::{self, File, OpenOptions};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::watch;

use super::error::ModelError;
use super::manifest::{ModelFile, Pack};
use super::storage;

const ATTEMPTS_PER_SOURCE: u32 = 3;
const CONNECT_TIMEOUT: Duration = Duration::from_secs(15);
const STALL_TIMEOUT: Duration = Duration::from_secs(30);
const RETRY_DELAY: Duration = Duration::from_millis(500);
const HASH_BUFFER: usize = 1024 * 1024;

pub type Cancel = watch::Receiver<bool>;

pub fn client() -> Result<Client, ModelError> {
  Ok(
    Client::builder()
      .user_agent(concat!("VoxMelody/", env!("CARGO_PKG_VERSION")))
      .connect_timeout(CONNECT_TIMEOUT)
      .read_timeout(STALL_TIMEOUT)
      .build()?,
  )
}

/// Resolves with `Paused` once cancellation is requested.
async fn cancelled(cancel: &mut Cancel) -> ModelError {
  let _ = cancel.wait_for(|stop| *stop).await;
  ModelError::Paused
}

/// Downloads every missing file. `on_progress` receives the pack's total bytes.
pub async fn download_pack(
  client: &Client,
  root: &Path,
  pack: &Pack,
  mut cancel: Cancel,
  on_progress: impl Fn(u64),
) -> Result<(), ModelError> {
  let mut finished = 0;
  for file in &pack.files {
    if !storage::is_installed(root, pack, file) {
      download_file(client, root, pack, file, &mut cancel, &|bytes| on_progress(finished + bytes)).await?;
    }
    finished += file.size;
    on_progress(finished);
  }
  Ok(())
}

async fn download_file(
  client: &Client,
  root: &Path,
  pack: &Pack,
  file: &ModelFile,
  cancel: &mut Cancel,
  on_progress: &impl Fn(u64),
) -> Result<(), ModelError> {
  let target = storage::file_path(root, pack, file);
  let part = storage::part_path(&target);
  if let Some(parent) = target.parent() {
    fs::create_dir_all(parent).await?;
  }

  let mut last_error = ModelError::Network;
  for source in &pack.sources {
    for attempt in 0..ATTEMPTS_PER_SOURCE {
      if attempt > 0 {
        tokio::select! {
          _ = tokio::time::sleep(RETRY_DELAY * attempt) => {}
          paused = cancelled(cancel) => return Err(paused),
        }
      }
      let url = format!("{source}{}", file.path);
      match fetch(client, &url, &part, file, cancel, on_progress).await {
        Ok(()) => {
          fs::rename(&part, &target).await?;
          return Ok(());
        }
        Err(error @ (ModelError::Network | ModelError::Corrupt)) => last_error = error,
        Err(error) => return Err(error),
      }
    }
  }
  Err(last_error)
}

/// Fetches or resumes one file, leaving a verified `.part` on success.
async fn fetch(
  client: &Client,
  url: &str,
  part: &Path,
  file: &ModelFile,
  cancel: &mut Cancel,
  on_progress: &impl Fn(u64),
) -> Result<(), ModelError> {
  let (mut hasher, mut written) = hash_existing(part, file.size, cancel).await?;

  if written < file.size {
    let mut request = client.get(url);
    if written > 0 {
      request = request.header(RANGE, format!("bytes={written}-"));
    }
    let mut response = tokio::select! {
      response = request.send() => response?,
      paused = cancelled(cancel) => return Err(paused),
    };
    let resumed = match response.status() {
      StatusCode::PARTIAL_CONTENT => {
        let expected = format!("bytes {written}-{}/{}", file.size - 1, file.size);
        let matches = response.headers().get(CONTENT_RANGE).is_some_and(|value| value == expected.as_str());
        if !matches {
          return Err(ModelError::Network);
        }
        true
      }
      StatusCode::OK => false,
      status => {
        log::warn!("unexpected status {status} for {}", file.path);
        return Err(ModelError::Network);
      }
    };
    if !resumed {
      hasher = Sha256::new();
      written = 0;
    }

    let mut output =
      OpenOptions::new().create(true).write(true).append(resumed).truncate(!resumed).open(part).await?;
    on_progress(written);
    let mut outcome = Ok(());
    loop {
      // Pausing only between writes means the file is closed when this returns.
      let next = tokio::select! {
        next = response.chunk() => next,
        paused = cancelled(cancel) => {
          outcome = Err(paused);
          break;
        }
      };
      let chunk = match next {
        Ok(Some(chunk)) => chunk,
        Ok(None) => break,
        Err(error) => {
          outcome = Err(error.into());
          break;
        }
      };
      if written + chunk.len() as u64 > file.size {
        outcome = Err(ModelError::Corrupt);
        break;
      }
      output.write_all(&chunk).await?;
      hasher.update(&chunk);
      written += chunk.len() as u64;
      on_progress(written);
    }
    output.flush().await?;
    output.sync_all().await?;
    drop(output);
    if outcome == Err(ModelError::Corrupt) {
      fs::remove_file(part).await?;
    }
    outcome?;
    if written < file.size {
      return Err(ModelError::Network);
    }
  }

  if hex::encode(hasher.finalize()) != file.sha256 {
    log::warn!("checksum mismatch for {}", file.path);
    fs::remove_file(part).await?;
    return Err(ModelError::Corrupt);
  }
  Ok(())
}

async fn hash_existing(part: &Path, limit: u64, cancel: &Cancel) -> Result<(Sha256, u64), ModelError> {
  let mut hasher = Sha256::new();
  let mut input = match File::open(part).await {
    Ok(input) => input,
    Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok((hasher, 0)),
    Err(error) => return Err(error.into()),
  };
  if input.metadata().await?.len() > limit {
    drop(input);
    fs::remove_file(part).await?;
    return Ok((hasher, 0));
  }
  let mut buffer = vec![0; HASH_BUFFER];
  let mut total = 0;
  loop {
    if *cancel.borrow() {
      return Err(ModelError::Paused);
    }
    let read = input.read(&mut buffer).await?;
    if read == 0 {
      return Ok((hasher, total));
    }
    hasher.update(&buffer[..read]);
    total += read as u64;
  }
}
