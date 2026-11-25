use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, SystemTime};

const STALE_AFTER: Duration = Duration::from_secs(24 * 60 * 60);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TakeKind {
  Preview,
  Full,
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
    let is_take = entry.path().extension().is_some_and(|extension| extension == "wav");
    if is_take && matches!(age, Ok(Ok(age)) if age > STALE_AFTER) {
      let _ = std::fs::remove_file(entry.path());
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn keeps_only_the_newest_of_each_kind() {
    let dir = tempfile::tempdir().unwrap();
    let takes = Takes::new(dir.path().into()).unwrap();
    let ids: Vec<u64> = [TakeKind::Full, TakeKind::Preview, TakeKind::Full]
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
