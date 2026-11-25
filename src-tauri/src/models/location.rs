use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use super::error::ModelError;
use super::manifest::Pack;
use super::storage;

pub const FOLDER_NAME: &str = "VoxMelody Models";
const COPY_BUFFER: usize = 1024 * 1024;

#[derive(Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
struct Settings {
  models_dir: Option<PathBuf>,
}

/// Remembers where models live; falls back to the default folder.
pub struct LocationStore {
  file: PathBuf,
  default_dir: PathBuf,
}

impl LocationStore {
  pub fn new(file: PathBuf, default_dir: PathBuf) -> Self {
    Self { file, default_dir }
  }

  pub fn default_dir(&self) -> &Path {
    &self.default_dir
  }

  pub fn load(&self) -> PathBuf {
    std::fs::read(&self.file)
      .ok()
      .and_then(|data| serde_json::from_slice::<Settings>(&data).ok())
      .and_then(|settings| settings.models_dir)
      .unwrap_or_else(|| self.default_dir.clone())
  }

  pub fn save(&self, dir: &Path) -> Result<(), ModelError> {
    let models_dir = (dir != self.default_dir).then(|| dir.to_path_buf());
    let data = serde_json::to_vec_pretty(&Settings { models_dir }).map_err(|_| ModelError::Storage)?;
    if let Some(parent) = self.file.parent() {
      std::fs::create_dir_all(parent)?;
    }
    let temporary = storage::part_path(&self.file);
    std::fs::write(&temporary, data)?;
    Ok(std::fs::rename(temporary, &self.file)?)
  }
}

/// Keeps models in a folder of their own inside the chosen one.
pub fn resolve(chosen: &Path) -> PathBuf {
  if chosen.file_name().is_some_and(|name| name == FOLDER_NAME) {
    chosen.to_path_buf()
  } else {
    chosen.join(FOLDER_NAME)
  }
}

struct Item {
  from: PathBuf,
  to: PathBuf,
  size: u64,
}

/// Every finished or partial file under `from` that `to` does not already have.
fn missing_items(from: &Path, to: &Path, packs: &[Pack]) -> Vec<Item> {
  let mut items = Vec::new();
  for pack in packs {
    for file in &pack.files {
      if storage::is_installed(to, pack, file) {
        continue;
      }
      let (source, target) = (storage::file_path(from, pack, file), storage::file_path(to, pack, file));
      for (from, to) in [(storage::part_path(&source), storage::part_path(&target)), (source, target)] {
        if let Ok(meta) = std::fs::metadata(&from) {
          items.push(Item { from, to, size: meta.len() });
        }
      }
    }
  }
  items
}

/// Copies models to `to` and checks every byte. The originals stay untouched.
pub async fn copy_models(
  from: &Path,
  to: &Path,
  packs: &[Pack],
  reserve: u64,
  on_progress: impl Fn(u64, u64),
) -> Result<(), ModelError> {
  let items = missing_items(from, to, packs);
  fs::create_dir_all(to).await?;
  let result = transfer(&items, to, reserve, on_progress).await;
  if result.is_err() {
    for item in &items {
      let _ = fs::remove_file(&item.to).await;
    }
  }
  result
}

async fn transfer(
  items: &[Item],
  to: &Path,
  reserve: u64,
  on_progress: impl Fn(u64, u64),
) -> Result<(), ModelError> {
  // Links are instant and need no space, but only work within one drive.
  let mut remaining = Vec::new();
  for item in items {
    if let Some(parent) = item.to.parent() {
      fs::create_dir_all(parent).await?;
    }
    if fs::hard_link(&item.from, &item.to).await.is_err() {
      remaining.push(item);
    }
  }
  let total: u64 = remaining.iter().map(|item| item.size).sum();
  if total > 0 && total + reserve > storage::free_space(to)? {
    return Err(ModelError::DiskFull);
  }
  let mut copied = 0;
  for item in remaining {
    copy_file(item, |bytes| on_progress(copied + bytes, total)).await?;
    copied += item.size;
  }
  Ok(())
}

async fn copy_file(item: &Item, on_progress: impl Fn(u64)) -> Result<(), ModelError> {
  let (mut input, mut output) = (fs::File::open(&item.from).await?, fs::File::create(&item.to).await?);
  let (mut buffer, mut written, mut hasher) = (vec![0; COPY_BUFFER], 0, Sha256::new());
  loop {
    let read = input.read(&mut buffer).await?;
    if read == 0 {
      break;
    }
    output.write_all(&buffer[..read]).await?;
    hasher.update(&buffer[..read]);
    written += read as u64;
    on_progress(written);
  }
  output.flush().await?;
  output.sync_all().await?;
  drop(output);
  if hasher.finalize() != hash_file(&item.to, &mut buffer).await? {
    return Err(ModelError::Corrupt);
  }
  Ok(())
}

async fn hash_file(path: &Path, buffer: &mut [u8]) -> Result<sha2::digest::Output<Sha256>, ModelError> {
  let (mut input, mut hasher) = (fs::File::open(path).await?, Sha256::new());
  loop {
    let read = input.read(buffer).await?;
    if read == 0 {
      return Ok(hasher.finalize());
    }
    hasher.update(&buffer[..read]);
  }
}

/// Deletes only the pack folders, never the user's own files around them.
pub async fn remove_models(root: &Path, packs: &[Pack]) {
  for pack in packs {
    match fs::remove_dir_all(root.join(&pack.id)).await {
      Err(error) if error.kind() != std::io::ErrorKind::NotFound => log::warn!("old models not removed: {error}"),
      _ => {}
    }
  }
  // Succeeds only when the folder is empty.
  let _ = fs::remove_dir(root).await;
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::models::manifest::ModelFile;

  fn packs() -> Vec<Pack> {
    vec![Pack {
      id: "voice".into(),
      recommended_memory: 0,
      sources: vec![],
      files: ["a.bin", "sub/b.bin", "c.bin"]
        .map(|path| ModelFile { path: path.into(), size: 4, sha256: String::new() })
        .into(),
    }]
  }

  #[tokio::test]
  async fn copies_finished_and_partial_files_then_cleans_up() {
    let (old, new) = (tempfile::tempdir().unwrap(), tempfile::tempdir().unwrap());
    let (from, to) = (old.path().join("m"), new.path().join(FOLDER_NAME));
    std::fs::create_dir_all(from.join("voice/sub")).unwrap();
    std::fs::write(from.join("voice/a.bin"), "aaaa").unwrap();
    std::fs::write(from.join("voice/sub/b.bin"), "bbbb").unwrap();
    std::fs::write(from.join("voice/c.bin.part"), "c").unwrap();
    std::fs::write(from.join("mine.txt"), "keep").unwrap();
    copy_models(&from, &to, &packs(), 0, |_, _| {}).await.unwrap();
    assert_eq!(std::fs::read(to.join("voice/sub/b.bin")).unwrap(), b"bbbb");
    assert_eq!(std::fs::read(to.join("voice/c.bin.part")).unwrap(), b"c");
    assert!(from.join("voice/a.bin").exists());
    remove_models(&from, &packs()).await;
    assert!(!from.join("voice").exists() && from.join("mine.txt").exists());
  }

  #[tokio::test]
  async fn keeps_models_already_at_the_destination() {
    let (from, to) = (tempfile::tempdir().unwrap(), tempfile::tempdir().unwrap());
    for dir in [&from, &to] {
      std::fs::create_dir_all(dir.path().join("voice")).unwrap();
    }
    std::fs::write(from.path().join("voice/a.bin"), "oldd").unwrap();
    std::fs::write(to.path().join("voice/a.bin"), "neww").unwrap();
    copy_models(from.path(), to.path(), &packs(), 0, |_, _| {}).await.unwrap();
    assert_eq!(std::fs::read(to.path().join("voice/a.bin")).unwrap(), b"neww");
  }

  #[tokio::test]
  async fn refuses_when_destination_is_full() {
    let (from, to) = (tempfile::tempdir().unwrap(), tempfile::tempdir().unwrap());
    std::fs::create_dir_all(from.path().join("voice")).unwrap();
    std::fs::write(from.path().join("voice/a.bin"), "aaaa").unwrap();
    // An existing partial file forces a copy instead of a link.
    std::fs::create_dir_all(to.path().join("voice")).unwrap();
    std::fs::write(to.path().join("voice/a.bin"), "a").unwrap();
    let result = copy_models(from.path(), to.path(), &packs(), u64::MAX / 2, |_, _| {}).await;
    assert_eq!(result, Err(ModelError::DiskFull));
    assert!(from.path().join("voice/a.bin").exists());
  }

  #[tokio::test]
  async fn copies_and_verifies_when_linking_is_impossible() {
    let (from, to) = (tempfile::tempdir().unwrap(), tempfile::tempdir().unwrap());
    for dir in [&from, &to] {
      std::fs::create_dir_all(dir.path().join("voice")).unwrap();
    }
    std::fs::write(from.path().join("voice/a.bin"), "aaaa").unwrap();
    std::fs::write(to.path().join("voice/a.bin"), "x").unwrap();
    copy_models(from.path(), to.path(), &packs(), 0, |_, _| {}).await.unwrap();
    assert_eq!(std::fs::read(to.path().join("voice/a.bin")).unwrap(), b"aaaa");
  }

  #[test]
  fn remembers_location_and_survives_bad_settings() {
    let dir = tempfile::tempdir().unwrap();
    let store = LocationStore::new(dir.path().join("c/settings.json"), dir.path().join("default"));
    assert_eq!(store.load(), dir.path().join("default"));
    store.save(&dir.path().join("custom")).unwrap();
    assert_eq!(store.load(), dir.path().join("custom"));
    std::fs::write(dir.path().join("c/settings.json"), "{broken").unwrap();
    assert_eq!(store.load(), dir.path().join("default"));
  }

  #[test]
  fn uses_its_own_folder() {
    assert_eq!(resolve(Path::new("D:/x")), Path::new("D:/x").join(FOLDER_NAME));
    assert_eq!(resolve(&Path::new("D:/x").join(FOLDER_NAME)), Path::new("D:/x").join(FOLDER_NAME));
  }
}
