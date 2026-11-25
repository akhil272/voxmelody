use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use sha2::{Digest, Sha256};

use super::super::manifest::{ModelFile, PackInfo};
use super::*;

#[derive(Clone, Copy, PartialEq)]
enum Mode {
  Good,
  Corrupt,
  Missing,
  IgnoresRange,
  DropsFirstConnection,
  Slow,
  Oversized,
}

struct Server {
  base: String,
  ranges: Arc<std::sync::Mutex<Vec<Option<u64>>>>,
}

fn serve(body: Vec<u8>, mode: Mode) -> Server {
  let listener = TcpListener::bind("127.0.0.1:0").unwrap();
  let base = format!("http://{}/", listener.local_addr().unwrap());
  let ranges = Arc::new(std::sync::Mutex::new(Vec::new()));
  let seen = ranges.clone();
  let count = AtomicUsize::new(0);
  std::thread::spawn(move || {
    for stream in listener.incoming() {
      let mut stream = stream.unwrap();
      let mut range = None;
      for line in BufReader::new(stream.try_clone().unwrap()).lines() {
        let line = line.unwrap_or_default().to_lowercase();
        if line.is_empty() {
          break;
        }
        if let Some(value) = line.strip_prefix("range: bytes=") {
          range = value.trim_end_matches('-').parse::<u64>().ok();
        }
      }
      seen.lock().unwrap().push(range);
      let first = count.fetch_add(1, Ordering::SeqCst) == 0;
      if mode == Mode::Missing {
        let _ = stream.write_all(b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n");
        continue;
      }
      let mut data = body.clone();
      match mode {
        Mode::Corrupt => data[0] ^= 0xff,
        Mode::Oversized => data.extend_from_slice(&[0; 64]),
        _ => {}
      }
      let start = if mode == Mode::IgnoresRange { 0 } else { range.unwrap_or(0) as usize };
      let head = if start > 0 {
        format!("HTTP/1.1 206 Partial Content\r\nContent-Range: bytes {start}-{}/{}\r\n", data.len() - 1, data.len())
      } else {
        "HTTP/1.1 200 OK\r\n".into()
      };
      let slice = &data[start..];
      let _ = stream.write_all(format!("{head}Content-Length: {}\r\nConnection: close\r\n\r\n", slice.len()).as_bytes());
      if mode == Mode::DropsFirstConnection && first {
        let _ = stream.write_all(&slice[..slice.len() / 2]);
      } else if mode == Mode::Slow {
        for piece in slice.chunks(1000) {
          if stream.write_all(piece).is_err() {
            break;
          }
          std::thread::sleep(Duration::from_millis(50));
        }
      } else {
        let _ = stream.write_all(slice);
      }
    }
  });
  Server { base, ranges }
}

fn body() -> Vec<u8> {
  (0..200_000u32).map(|n| (n % 251) as u8).collect()
}

fn pack(body: &[u8], sources: &[&Server]) -> &'static Pack {
  Box::leak(Box::new(Pack {
    info: PackInfo { id: "test".into(), ..Default::default() },
    sources: sources.iter().map(|server| server.base.clone()).collect(),
    files: vec![ModelFile {
      path: "nested/file.bin".into(),
      size: body.len() as u64,
      sha256: hex::encode(Sha256::digest(body)),
    }],
  }))
}

fn manager(root: &Path, listener: StatusListener) -> ModelManager {
  let store = LocationStore::new(root.join("settings.json"), root.to_path_buf());
  ModelManager::new(store, listener, Arc::new(|_| {})).unwrap()
}

async fn run(root: &Path, pack: &Pack) -> Result<(), ModelError> {
  let (_keep, cancel) = watch::channel(false);
  download::download_pack(&download::client().unwrap(), root, pack, cancel, |_| {}).await
}

fn target(root: &Path) -> PathBuf {
  root.join("test/nested/file.bin")
}

#[tokio::test]
async fn downloads_and_verifies() {
  let (dir, data) = (tempfile::tempdir().unwrap(), body());
  let pack = pack(&data, &[&serve(data.clone(), Mode::Good)]);
  assert_eq!(run(dir.path(), pack).await, Ok(()));
  assert_eq!(std::fs::read(target(dir.path())).unwrap(), data);
  assert!(!storage::part_path(&target(dir.path())).exists());
  assert!(storage::is_pack_installed(dir.path(), pack));
}

#[tokio::test]
async fn resumes_from_partial_file() {
  let (dir, data) = (tempfile::tempdir().unwrap(), body());
  let server = serve(data.clone(), Mode::Good);
  let part = storage::part_path(&target(dir.path()));
  std::fs::create_dir_all(part.parent().unwrap()).unwrap();
  std::fs::write(&part, &data[..150_000]).unwrap();
  assert_eq!(run(dir.path(), pack(&data, &[&server])).await, Ok(()));
  assert_eq!(*server.ranges.lock().unwrap(), vec![Some(150_000)]);
  assert_eq!(std::fs::read(target(dir.path())).unwrap(), data);
}

#[tokio::test]
async fn recovers_from_dropped_connection() {
  let (dir, data) = (tempfile::tempdir().unwrap(), body());
  let server = serve(data.clone(), Mode::DropsFirstConnection);
  assert_eq!(run(dir.path(), pack(&data, &[&server])).await, Ok(()));
  assert_eq!(std::fs::read(target(dir.path())).unwrap(), data);
  assert!(server.ranges.lock().unwrap().len() >= 2);
}

#[tokio::test]
async fn restarts_when_server_ignores_range() {
  let (dir, data) = (tempfile::tempdir().unwrap(), body());
  let part = storage::part_path(&target(dir.path()));
  std::fs::create_dir_all(part.parent().unwrap()).unwrap();
  std::fs::write(&part, &data[..1000]).unwrap();
  assert_eq!(run(dir.path(), pack(&data, &[&serve(data.clone(), Mode::IgnoresRange)])).await, Ok(()));
  assert_eq!(std::fs::read(target(dir.path())).unwrap(), data);
}

#[tokio::test]
async fn falls_back_to_next_source() {
  for bad in [Mode::Missing, Mode::Corrupt, Mode::Oversized] {
    let (dir, data) = (tempfile::tempdir().unwrap(), body());
    let (first, second) = (serve(data.clone(), bad), serve(data.clone(), Mode::Good));
    assert_eq!(run(dir.path(), pack(&data, &[&first, &second])).await, Ok(()));
    assert_eq!(std::fs::read(target(dir.path())).unwrap(), data);
  }
}

#[tokio::test]
async fn never_installs_corrupt_files() {
  let (dir, data) = (tempfile::tempdir().unwrap(), body());
  let pack = pack(&data, &[&serve(data.clone(), Mode::Corrupt)]);
  assert_eq!(run(dir.path(), pack).await, Err(ModelError::Corrupt));
  assert!(!target(dir.path()).exists());
  assert_eq!(storage::pack_progress(dir.path(), pack), 0);
}

#[tokio::test]
async fn discards_tampered_partial_file() {
  let (dir, data) = (tempfile::tempdir().unwrap(), body());
  let part = storage::part_path(&target(dir.path()));
  std::fs::create_dir_all(part.parent().unwrap()).unwrap();
  std::fs::write(&part, vec![9u8; 5000]).unwrap();
  assert_eq!(run(dir.path(), pack(&data, &[&serve(data.clone(), Mode::Good)])).await, Ok(()));
  assert_eq!(std::fs::read(target(dir.path())).unwrap(), data);
}

#[test]
fn pauses_resumes_and_removes() {
  let (dir, data) = (tempfile::tempdir().unwrap(), body());
  let slow = serve(data.clone(), Mode::Slow);
  let pack = pack(&data, &[&slow]);
  let events = Arc::new(std::sync::Mutex::new(Vec::new()));
  let sink = events.clone();
  let manager = manager(dir.path(), Arc::new(move |event| sink.lock().unwrap().push(event)));
  async_runtime::block_on(async {
    manager.start(pack).await.unwrap();
    manager.start(pack).await.unwrap();
    tokio::time::sleep(Duration::from_millis(1500)).await;
    assert_eq!(manager.status(pack).await.state, PackState::Downloading);
    manager.pause(pack).await;
    let paused = manager.status(pack).await;
    assert_eq!(paused.state, PackState::Paused);
    assert!(paused.downloaded_bytes > 0 && paused.error.is_none());
    assert_eq!(slow.ranges.lock().unwrap().len(), 1, "second start must be ignored");

    manager.start(pack).await.unwrap();
    tokio::time::sleep(Duration::from_millis(500)).await;
    assert_eq!(slow.ranges.lock().unwrap()[1], Some(paused.downloaded_bytes));

    manager.remove(pack).await.unwrap();
    assert_eq!(manager.status(pack).await.state, PackState::Available);
    assert!(!dir.path().join("test").exists());
  });
  assert_eq!(events.lock().unwrap().last().unwrap().state, PackState::Available);
}

#[test]
fn refuses_when_disk_is_too_small() {
  let dir = tempfile::tempdir().unwrap();
  let pack: &'static Pack = Box::leak(Box::new(Pack {
    info: PackInfo { id: "huge".into(), ..Default::default() },
    sources: vec!["http://127.0.0.1:1/".into()],
    files: vec![ModelFile { path: "a".into(), size: u64::MAX / 4, sha256: "0".repeat(64) }],
  }));
  let manager = manager(dir.path(), Arc::new(|_| {}));
  assert_eq!(async_runtime::block_on(manager.start(pack)), Err(ModelError::DiskFull));
}

#[test]
fn missing_custom_location_blocks_downloads() {
  let dir = tempfile::tempdir().unwrap();
  let gone = dir.path().join("unplugged");
  let store = LocationStore::new(dir.path().join("settings.json"), dir.path().join("default"));
  store.save(&gone).unwrap();
  let manager = ModelManager::new(store, Arc::new(|_| {}), Arc::new(|_| {})).unwrap();
  assert!(!manager.location().available);
  assert_eq!(async_runtime::block_on(manager.start(&manifest::packs()[0])), Err(ModelError::LocationMissing));
  assert!(!gone.exists(), "must not recreate the folder");
}

#[test]
fn changes_location_and_restores_default() {
  let dir = tempfile::tempdir().unwrap();
  let default = dir.path().join("default");
  let store = LocationStore::new(dir.path().join("settings.json"), default.clone());
  let manager = ModelManager::new(store, Arc::new(|_| {}), Arc::new(|_| {})).unwrap();
  let partial = default.join("voice/LICENSE.part");
  std::fs::create_dir_all(partial.parent().unwrap()).unwrap();
  std::fs::write(&partial, "x").unwrap();
  async_runtime::block_on(async {
    manager.change_location(&dir.path().join("drive")).await.unwrap();
    assert!(!manager.location().is_default && manager.location().available);
    assert!(dir.path().join("drive").join(location::FOLDER_NAME).join("voice/LICENSE.part").exists());
    assert!(!partial.exists());
    assert_eq!(
      manager.change_location(&dir.path().join("drive").join(location::FOLDER_NAME).join("inner")).await,
      Err(ModelError::Storage)
    );
    manager.restore_location().await.unwrap();
  });
  assert!(manager.location().is_default && partial.exists());
}

#[test]
fn shares_files_between_packs() {
  let dir = tempfile::tempdir().unwrap();
  let file = |path: &str, sha256: &str| ModelFile { path: path.into(), size: 4, sha256: sha256.into() };
  let pack = |id: &str, files| Pack { info: PackInfo { id: id.into(), ..Default::default() }, sources: vec![], files };
  let packs =
    [pack("one", vec![file("a", "x"), file("b", "y")]), pack("two", vec![file("sub/c", "x"), file("d", "z")])];
  for name in ["a", "b"] {
    let path = storage::file_path(dir.path(), &packs[0], &file(name, ""));
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, "data").unwrap();
  }
  assert_eq!(storage::download_bytes(dir.path(), &packs[1], &packs), 4);
  let shared = dir.path().join("two/sub/c");
  std::fs::create_dir_all(shared.parent().unwrap()).unwrap();
  std::fs::write(storage::part_path(&shared), "d").unwrap();
  storage::link_shared(dir.path(), &packs[1], &packs);
  assert_eq!(storage::pack_progress(dir.path(), &packs[1]), 4);
  assert!(!storage::part_path(&shared).exists());

  // Removing one pack leaves the other whole.
  std::fs::remove_dir_all(dir.path().join("one")).unwrap();
  assert_eq!(std::fs::read(shared).unwrap(), b"data");
}
