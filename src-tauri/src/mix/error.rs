use serde::Serialize;

use crate::library::error::LibraryError;

/// Stable codes the UI maps to user-facing copy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum MixError {
  /// A chosen recording was moved or deleted.
  Missing,
  /// A chosen recording has no usable sound.
  Audio,
  DiskFull,
  Output,
  Request,
}

impl From<LibraryError> for MixError {
  fn from(error: LibraryError) -> Self {
    match error {
      LibraryError::Missing | LibraryError::Mismatch => Self::Missing,
      LibraryError::DiskFull => Self::DiskFull,
      LibraryError::Storage => Self::Output,
    }
  }
}

impl From<std::io::Error> for MixError {
  fn from(error: std::io::Error) -> Self {
    match error.kind() {
      std::io::ErrorKind::InvalidData => Self::Audio,
      _ => LibraryError::from(error).into(),
    }
  }
}
