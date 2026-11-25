use serde::Serialize;

/// Stable codes the UI maps to user-facing copy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum LibraryError {
  /// The recording's file was moved or deleted outside the app.
  Missing,
  /// The chosen file is not this recording.
  Mismatch,
  DiskFull,
  Storage,
}

impl From<std::io::Error> for LibraryError {
  fn from(error: std::io::Error) -> Self {
    log::warn!("library error: {error}");
    match error.kind() {
      std::io::ErrorKind::NotFound => Self::Missing,
      std::io::ErrorKind::StorageFull => Self::DiskFull,
      _ => Self::Storage,
    }
  }
}
