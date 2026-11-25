use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, SystemTime};

use serde::Serialize;
use tauri::ipc::Response;
use tauri::{AppHandle, Manager, State};
use tauri_plugin_dialog::DialogExt;

const STALE_AFTER: Duration = Duration::from_secs(24 * 60 * 60);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TakeKind {
  Preview,
  Voice,
  Music,
}

/// Stable codes the UI maps to user-facing copy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum TakeError {
  Output,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Take {
  pub id: u64,
  pub duration_ms: u64,
  pub peaks: Vec<f32>,
}

pub fn setup(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
  app.manage(Takes::new(crate::models::commands::data_dir(app)?.join("cache").join("takes"))?);
  Ok(())
}

/// Temporary recordings: only the newest of each kind is kept.
pub struct Takes {
  dir: PathBuf,
  next: AtomicU64,
  current: Mutex<HashMap<TakeKind, u64>>,
}

impl Takes {
  pub fn new(dir: PathBuf) -> std::io::Result<Self> {
    std::fs::create_dir_all(&dir)?;
    remove_stale(&dir);
    Ok(Self { dir, next: AtomicU64::new(1), current: Mutex::default() })
  }

  /// The process id keeps two open windows of the app apart.
  fn path(&self, id: u64) -> PathBuf {
    self.dir.join(format!("{}-{id}.wav", std::process::id()))
  }

  pub fn reserve(&self) -> (u64, PathBuf) {
    let id = self.next.fetch_add(1, Ordering::Relaxed);
    (id, self.path(id))
  }

  pub fn discard(&self, id: u64) {
    let _ = std::fs::remove_file(self.path(id));
  }

  pub fn keep(&self, kind: TakeKind, id: u64) {
    let previous = self.current.lock().ok().and_then(|mut current| current.insert(kind, id));
    if let Some(previous) = previous {
      self.discard(previous);
    }
  }

  /// Path of a current take; anything else is refused.
  pub fn find(&self, id: u64) -> Option<PathBuf> {
    let current = self.current.lock().ok()?;
    current.values().any(|value| *value == id).then(|| self.path(id))
  }
}

fn remove_stale(dir: &Path) {
  let Ok(entries) = std::fs::read_dir(dir) else { return };
  for entry in entries.flatten() {
    let age = entry.metadata().and_then(|meta| meta.modified()).map(|time| SystemTime::now().duration_since(time));
    let is_take = entry.path().extension().is_some_and(|extension| extension == "wav" || extension == "raw");
    if is_take && matches!(age, Ok(Ok(age)) if age > STALE_AFTER) {
      let _ = std::fs::remove_file(entry.path());
    }
  }
}

#[tauri::command]
pub async fn read_take(takes: State<'_, Takes>, id: u64) -> Result<Response, TakeError> {
  let path = takes.find(id).ok_or(TakeError::Output)?;
  Ok(Response::new(tokio::fs::read(path).await.map_err(|_| TakeError::Output)?))
}

/// Returns false when the user closes the save dialog.
#[tauri::command]
pub async fn export_take(
  app: AppHandle,
  takes: State<'_, Takes>,
  id: u64,
  name: String,
) -> Result<bool, TakeError> {
  let source = takes.find(id).ok_or(TakeError::Output)?;
  let (sender, receiver) = tokio::sync::oneshot::channel();
  app.dialog().file().set_file_name(name).add_filter("WAV", &["wav"]).save_file(move |file| {
    let _ = sender.send(file);
  });
  let Ok(Some(file)) = receiver.await else { return Ok(false) };
  let mut target = file.into_path().map_err(|_| TakeError::Output)?;
  if target.extension().is_none() {
    target.set_extension("wav");
  }
  tokio::fs::copy(source, target).await.map_err(|_| TakeError::Output)?;
  Ok(true)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn keeps_only_the_newest_of_each_kind() {
    let dir = tempfile::tempdir().unwrap();
    let takes = Takes::new(dir.path().into()).unwrap();
    let ids: Vec<u64> = [TakeKind::Voice, TakeKind::Preview, TakeKind::Voice]
      .map(|kind| {
        let (id, path) = takes.reserve();
        std::fs::write(path, "x").unwrap();
        takes.keep(kind, id);
        id
      })
      .into();
    assert!(takes.find(ids[0]).is_none());
    assert!(takes.find(ids[1]).unwrap().exists() && takes.find(ids[2]).unwrap().exists());
    assert_eq!(std::fs::read_dir(dir.path()).unwrap().count(), 2);
    assert!(takes.find(999).is_none());
  }
}
