use serde::Serialize;

/// Stable codes the UI maps to user-facing copy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum CloneError {
  TooShort,
  TooQuiet,
  Audio,
  Consent,
  Name,
  Missing,
  Storage,
}

impl From<std::io::Error> for CloneError {
  fn from(error: std::io::Error) -> Self {
    log::warn!("cloned voice storage error: {error}");
    match error.kind() {
      std::io::ErrorKind::NotFound => Self::Missing,
      _ => Self::Storage,
    }
  }
}
