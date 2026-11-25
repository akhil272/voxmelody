use std::io::{BufWriter, Write};
use std::path::Path;

use super::CHANNELS;

const HEADER_BYTES: u32 = 36;
const SAMPLE_BYTES: u32 = 2;

/// Writes interleaved stereo as 16-bit PCM.
pub fn write(path: &Path, samples: &[f32], rate: u32) -> std::io::Result<()> {
  let frame_bytes = CHANNELS as u32 * SAMPLE_BYTES;
  let data_bytes = samples.len() as u32 * SAMPLE_BYTES;
  let mut file = BufWriter::new(std::fs::File::create(path)?);
  file.write_all(b"RIFF")?;
  file.write_all(&(HEADER_BYTES + data_bytes).to_le_bytes())?;
  file.write_all(b"WAVEfmt ")?;
  file.write_all(&16u32.to_le_bytes())?;
  file.write_all(&1u16.to_le_bytes())?;
  file.write_all(&(CHANNELS as u16).to_le_bytes())?;
  file.write_all(&rate.to_le_bytes())?;
  file.write_all(&(rate * frame_bytes).to_le_bytes())?;
  file.write_all(&(frame_bytes as u16).to_le_bytes())?;
  file.write_all(&(SAMPLE_BYTES as u16 * 8).to_le_bytes())?;
  file.write_all(b"data")?;
  file.write_all(&data_bytes.to_le_bytes())?;
  for sample in samples {
    file.write_all(&((sample.clamp(-1.0, 1.0) * f32::from(i16::MAX)).round() as i16).to_le_bytes())?;
  }
  file.flush()
}

/// Reads the raw little-endian float samples an engine wrote.
pub fn read_raw(path: &Path) -> std::io::Result<Vec<f32>> {
  let data = std::fs::read(path)?;
  Ok(data.as_chunks::<4>().0.iter().map(|bytes| f32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])).collect())
}

/// Loudest sample in each of at most `count` slices, for drawing a waveform.
pub fn peaks(samples: &[f32], count: usize) -> Vec<f32> {
  let size = samples.len().div_ceil(count).max(1);
  samples.chunks(size).map(|slice| slice.iter().fold(0.0f32, |peak, sample| peak.max(sample.abs()))).collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn writes_a_stereo_header() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("é.wav");
    write(&path, &[0.0, 1.0, -2.0, 0.5], 48_000).unwrap();
    let data = std::fs::read(&path).unwrap();
    assert_eq!(data.len(), 52);
    assert_eq!(&data[22..24], &2u16.to_le_bytes());
    assert_eq!(&data[28..32], &192_000u32.to_le_bytes());
    assert_eq!(&data[48..50], &(-i16::MAX).to_le_bytes());
  }

  #[test]
  fn reads_raw_samples_and_limits_peaks() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("a.raw");
    std::fs::write(&path, [0.25f32.to_le_bytes(), (-1.0f32).to_le_bytes()].concat()).unwrap();
    assert_eq!(read_raw(&path).unwrap(), vec![0.25, -1.0]);
    assert!(peaks(&vec![0.5; 1001], 160).len() <= 160);
    assert_eq!(peaks(&[0.1, -0.9], 160), vec![0.1, 0.9]);
  }
}
