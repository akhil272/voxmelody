use std::sync::OnceLock;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct ModelFile {
  pub path: String,
  pub size: u64,
  pub sha256: String,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PackKind {
  #[default]
  Voice,
  Music,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceModel {
  pub family: String,
  /// Main model file, where the family has one.
  pub file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MusicModel {
  pub lm: String,
  pub dit: String,
  pub steps: u32,
  /// True when the model responds to prompt strength.
  pub guidance: bool,
  /// Graphics memory below which the CPU is used.
  pub graphics_memory: u64,
}

/// What people see about a model.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackInfo {
  pub id: String,
  pub kind: PackKind,
  pub name: String,
  pub author: String,
  pub released: String,
  pub licence: String,
  /// True when the licence limits what the model may be used for.
  pub restricted: bool,
  pub page_url: String,
  pub parameters: String,
  pub precision: String,
  pub languages: Vec<String>,
  pub recommended_memory: u64,
  pub voice: Option<VoiceModel>,
  pub music: Option<MusicModel>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pack {
  #[serde(flatten)]
  pub info: PackInfo,
  /// Base URLs tried in order; each ends with a slash.
  pub sources: Vec<String>,
  pub files: Vec<ModelFile>,
}

#[derive(Debug, Deserialize)]
struct Manifest {
  packs: Vec<Pack>,
}

impl std::ops::Deref for Pack {
  type Target = PackInfo;

  fn deref(&self) -> &PackInfo {
    &self.info
  }
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
    let ids: std::collections::HashSet<_> = packs().iter().map(|pack| &pack.id).collect();
    assert_eq!(ids.len(), packs().len());
    for pack in packs() {
      assert!(pack.id.chars().all(|c| c.is_ascii_alphanumeric() || c == '-'), "{}", pack.id);
      assert_eq!(pack.voice.is_some(), pack.kind == PackKind::Voice);
      assert_eq!(pack.music.is_some(), pack.kind == PackKind::Music);
      assert!(!pack.author.is_empty() && !pack.licence.is_empty() && pack.released.len() == 10);
      assert!(pack.page_url.starts_with("https://"));
      let model_files = [pack.voice.as_ref().map(|v| &v.file), pack.music.as_ref().map(|m| &m.lm), pack.music.as_ref().map(|m| &m.dit)];
      for name in model_files.into_iter().flatten().filter(|name| !name.is_empty()) {
        assert!(pack.files.iter().any(|file| file.path == *name), "{name}");
      }
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
