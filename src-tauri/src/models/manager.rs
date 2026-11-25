use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex as SyncMutex, RwLock};
use std::time::{Duration, Instant};

use reqwest::Client;
use serde::Serialize;
use tauri::async_runtime::{self, JoinHandle};
use tokio::sync::{Mutex, watch};

use super::error::ModelError;
use super::location::{self, LocationStore};
use super::manifest::{self, Pack};
use super::{download, storage};

const DISK_RESERVE: u64 = 256 * 1024 * 1024;
const PROGRESS_INTERVAL: Duration = Duration::from_millis(200);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum PackState {
  Available,
  Downloading,
  Paused,
  Installed,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PackStatus {
  pub id: String,
  pub state: PackState,
  pub downloaded_bytes: u64,
  pub total_bytes: u64,
  pub recommended_memory: u64,
  pub error: Option<ModelError>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationStatus {
  pub path: String,
  pub is_default: bool,
  /// False when a custom folder is gone, such as on an unplugged drive.
  pub available: bool,
  pub move_percent: Option<u64>,
}

pub type LocationListener = Arc<dyn Fn(LocationStatus) + Send + Sync>;
pub type StatusListener = Arc<dyn Fn(PackStatus) + Send + Sync>;

struct Job {
  cancel: watch::Sender<bool>,
  handle: JoinHandle<()>,
}

pub struct ModelManager {
  root: RwLock<PathBuf>,
  locations: LocationStore,
  client: Client,
  listener: StatusListener,
  location_listener: LocationListener,
  jobs: Mutex<HashMap<String, Job>>,
}

impl ModelManager {
  pub fn new(
    locations: LocationStore,
    listener: StatusListener,
    location_listener: LocationListener,
  ) -> Result<Self, ModelError> {
    let root = RwLock::new(locations.load());
    Ok(Self { root, locations, client: download::client()?, listener, location_listener, jobs: Mutex::default() })
  }

  fn root(&self) -> PathBuf {
    self.root.read().unwrap_or_else(|poisoned| poisoned.into_inner()).clone()
  }

  fn set_root(&self, root: PathBuf) {
    *self.root.write().unwrap_or_else(|poisoned| poisoned.into_inner()) = root;
  }

  pub fn location(&self) -> LocationStatus {
    self.location_status(None)
  }

  fn location_status(&self, move_percent: Option<u64>) -> LocationStatus {
    let root = self.root();
    let is_default = root == self.locations.default_dir();
    LocationStatus {
      path: root.display().to_string(),
      is_default,
      available: is_default || root.exists(),
      move_percent,
    }
  }

  pub async fn change_location(&self, chosen: &Path) -> Result<(), ModelError> {
    self.relocate(location::resolve(chosen)).await
  }

  pub async fn restore_location(&self) -> Result<(), ModelError> {
    self.relocate(self.locations.default_dir().to_path_buf()).await
  }

  /// Copies, verifies and remembers the new folder before deleting the old one.
  async fn relocate(&self, to: PathBuf) -> Result<(), ModelError> {
    let mut jobs = self.jobs.lock().await;
    let from = self.root();
    if from == to {
      return Ok(());
    }
    if from.starts_with(&to) || to.starts_with(&from) {
      return Err(ModelError::Storage);
    }
    for (_, job) in jobs.drain() {
      stop(Some(job)).await;
    }

    (self.location_listener)(self.location_status(Some(0)));
    let last = SyncMutex::new(Instant::now());
    let report = |done: u64, total: u64| {
      throttled(&last, || (self.location_listener)(self.location_status(Some(done * 100 / total.max(1)))));
    };
    let copied = location::copy_models(&from, &to, manifest::packs(), DISK_RESERVE, report).await;
    let result = copied.and_then(|()| self.locations.save(&to));
    if result.is_ok() {
      self.set_root(to);
      location::remove_models(&from, manifest::packs()).await;
    }
    (self.location_listener)(self.location());
    for pack in manifest::packs() {
      (self.listener)(self.idle(pack, None));
    }
    result
  }

  pub fn free_space(&self) -> Result<u64, ModelError> {
    Ok(storage::free_space(&self.root())?)
  }

  pub async fn status(&self, pack: &Pack) -> PackStatus {
    let jobs = self.jobs.lock().await;
    let active = jobs.get(&pack.id).is_some_and(|job| !job.handle.inner().is_finished());
    if active { self.downloading(pack, storage::pack_progress(&self.root(), pack)) } else { self.idle(pack, None) }
  }

  /// Starts or resumes a pack. Does nothing if it is already downloading.
  pub async fn start(&self, pack: &'static Pack) -> Result<(), ModelError> {
    let mut jobs = self.jobs.lock().await;
    if jobs.get(&pack.id).is_some_and(|job| !job.handle.inner().is_finished()) {
      return Ok(());
    }
    if !self.location().available {
      return Err(ModelError::LocationMissing);
    }
    let missing = pack.total_bytes() - storage::pack_progress(&self.root(), pack);
    let reserved = self.pending_bytes(&jobs);
    if missing > 0 && missing + reserved + DISK_RESERVE > self.free_space()? {
      return Err(ModelError::DiskFull);
    }

    let (cancel, receiver) = watch::channel(false);
    let worker = Worker { root: self.root(), client: self.client.clone(), listener: self.listener.clone() };
    (self.listener)(self.downloading(pack, storage::pack_progress(&self.root(), pack)));
    let handle = async_runtime::spawn(worker.run(pack, receiver));
    jobs.insert(pack.id.clone(), Job { cancel, handle });
    Ok(())
  }

  /// Stops a download and waits until its files are closed.
  pub async fn pause(&self, pack: &Pack) {
    let mut jobs = self.jobs.lock().await;
    stop(jobs.remove(&pack.id)).await;
  }

  pub async fn remove(&self, pack: &Pack) -> Result<(), ModelError> {
    // Holding the lock keeps a new download from starting during deletion.
    let mut jobs = self.jobs.lock().await;
    stop(jobs.remove(&pack.id)).await;
    let result = match tokio::fs::remove_dir_all(self.root().join(&pack.id)).await {
      Err(error) if error.kind() != std::io::ErrorKind::NotFound => Err(error.into()),
      _ => Ok(()),
    };
    (self.listener)(self.idle(pack, result.err()));
    result
  }

  /// Bytes other active downloads still need.
  fn pending_bytes(&self, jobs: &HashMap<String, Job>) -> u64 {
    manifest::packs()
      .iter()
      .filter(|pack| jobs.get(&pack.id).is_some_and(|job| !job.handle.inner().is_finished()))
      .map(|pack| pack.total_bytes() - storage::pack_progress(&self.root(), pack))
      .sum()
  }

  fn downloading(&self, pack: &Pack, downloaded: u64) -> PackStatus {
    status(pack, PackState::Downloading, downloaded, None)
  }

  fn idle(&self, pack: &Pack, error: Option<ModelError>) -> PackStatus {
    idle_status(&self.root(), pack, error)
  }
}

/// Runs `action` at most once per progress interval.
fn throttled(last: &SyncMutex<Instant>, action: impl FnOnce()) {
  let Ok(mut last) = last.lock() else { return };
  if last.elapsed() >= PROGRESS_INTERVAL {
    *last = Instant::now();
    action();
  }
}

async fn stop(job: Option<Job>) {
  if let Some(job) = job {
    let _ = job.cancel.send(true);
    let _ = job.handle.await;
  }
}

fn status(pack: &Pack, state: PackState, downloaded_bytes: u64, error: Option<ModelError>) -> PackStatus {
  PackStatus {
    id: pack.id.clone(),
    state,
    downloaded_bytes,
    total_bytes: pack.total_bytes(),
    recommended_memory: pack.recommended_memory,
    error,
  }
}

fn idle_status(root: &Path, pack: &Pack, error: Option<ModelError>) -> PackStatus {
  let downloaded = storage::pack_progress(root, pack);
  let state = match downloaded {
    _ if storage::is_pack_installed(root, pack) => PackState::Installed,
    0 => PackState::Available,
    _ => PackState::Paused,
  };
  status(pack, state, downloaded, error)
}

struct Worker {
  root: PathBuf,
  client: Client,
  listener: StatusListener,
}

impl Worker {
  async fn run(self, pack: &'static Pack, cancel: download::Cancel) {
    let last = SyncMutex::new(Instant::now());
    let report = |bytes: u64| {
      throttled(&last, || (self.listener)(status(pack, PackState::Downloading, bytes, None)));
    };
    let result = download::download_pack(&self.client, &self.root, pack, cancel, report).await;
    let error = result.err().filter(|error| *error != ModelError::Paused);
    (self.listener)(idle_status(&self.root, pack, error));
  }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
