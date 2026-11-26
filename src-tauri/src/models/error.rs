use serde::Serialize;

/// Stable codes the UI maps to user-facing copy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ModelError {
  UnknownPack,
  Network,
  Corrupt,
  DiskFull,
  Storage,
  LocationMissing,
  Paused,
}

impl From<std::io::Error> for ModelError {
  fn from(error: std::io::Error) -> Self {
    log::warn!("model storage error: {error}");
    match error.kind() {
      std::io::ErrorKind::StorageFull => Self::DiskFull,
      _ => Self::Storage,
    }
  }
}

impl From<reqwest::Error> for ModelError {
  fn from(error: reqwest::Error) -> Self {
    log::warn!("model download error: {error}");
    Self::Network
  }
}
