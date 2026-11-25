use serde::{Deserialize, Serialize};

/// Stable codes the UI maps to user-facing copy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum VoiceError {
  NotInstalled,
  NoSpeech,
  TooLong,
  Model,
  Output,
  Request,
  Engine,
  Cancelled,
}

impl From<std::io::Error> for VoiceError {
  fn from(error: std::io::Error) -> Self {
    log::warn!("voice engine error: {error}");
    Self::Engine
  }
}
