use serde::{Deserialize, Serialize};

/// Stable codes the UI maps to user-facing copy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MusicError {
  NotInstalled,
  Model,
  Output,
  Request,
  Engine,
  Cancelled,
}

impl From<std::io::Error> for MusicError {
  fn from(error: std::io::Error) -> Self {
    log::warn!("music engine error: {error}");
    Self::Engine
  }
}
