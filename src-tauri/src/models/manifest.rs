use std::sync::OnceLock;

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct ModelFile {
  pub path: String,
  pub size: u64,
  pub sha256: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pack {
  pub id: String,
  pub recommended_memory: u64,
  /// Base URLs tried in order; each ends with a slash.
  pub sources: Vec<String>,
  pub files: Vec<ModelFile>,
}

#[derive(Debug, Deserialize)]
struct Manifest {
  packs: Vec<Pack>,
}

impl Pack {
  pub fn total_bytes(&self) -> u64 {
    self.files.iter().map(|file| file.size).sum()
  }
}

/// The manifest is compiled in, and a unit test parses it.
pub fn packs() -> &'static [Pack] {
  static MANIFEST: OnceLock<Manifest> = OnceLock::new();
  &MANIFEST
    .get_or_init(|| serde_json::from_str(include_str!("../../models.json")).expect("valid models.json"))
    .packs
}

pub fn find(id: &str) -> Option<&'static Pack> {
  packs().iter().find(|pack| pack.id == id)
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::path::{Component, Path};

  #[test]
  fn manifest_is_safe_and_complete() {
    assert_eq!(packs().len(), 2);
    for pack in packs() {
      assert!(!pack.sources.is_empty());
      assert!(pack.sources.iter().all(|s| s.starts_with("https://") && s.ends_with('/')));
      for file in &pack.files {
        assert_eq!(file.sha256.len(), 64, "{}", file.path);
        assert!(file.size > 0);
        assert!(Path::new(&file.path).components().all(|c| matches!(c, Component::Normal(_))));
      }
    }
  }
}
