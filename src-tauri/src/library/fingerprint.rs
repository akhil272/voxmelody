use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

const SAMPLE_BYTES: u64 = 64 * 1024;

/// Identifies a recording wherever it is moved. Size is not enough, because
/// tracks of the same length have the same size; reading it all is too slow.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Fingerprint {
  bytes: u64,
  hash: String,
}

impl Fingerprint {
  /// Hashes the start and the middle of the file.
  pub fn of(path: &Path) -> std::io::Result<Self> {
    let mut file = File::open(path)?;
    let bytes = file.metadata()?.len();
    let mut hasher = Sha256::new();
    for start in [0, bytes / 2] {
      let mut sample = Vec::new();
      file.seek(SeekFrom::Start(start))?;
      file.by_ref().take(SAMPLE_BYTES).read_to_end(&mut sample)?;
      hasher.update(&sample);
    }
    Ok(Self { bytes, hash: hex::encode(hasher.finalize()) })
  }

  pub fn matches(&self, path: &Path) -> bool {
    let same_size = std::fs::metadata(path).is_ok_and(|meta| meta.is_file() && meta.len() == self.bytes);
    same_size && Self::of(path).is_ok_and(|other| other == *self)
  }
}
