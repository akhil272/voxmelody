use std::io::{BufWriter, ErrorKind, Write};
use std::path::Path;

const HEADER_BYTES: u32 = 36;

/// Writes 16-bit mono PCM.
pub fn write(path: &Path, samples: &[f32], rate: u32) -> std::io::Result<()> {
  let data_bytes = samples.len() as u32 * 2;
  let mut file = BufWriter::new(std::fs::File::create(path)?);
  file.write_all(b"RIFF")?;
  file.write_all(&(HEADER_BYTES + data_bytes).to_le_bytes())?;
  file.write_all(b"WAVEfmt ")?;
  file.write_all(&16u32.to_le_bytes())?;
  file.write_all(&1u16.to_le_bytes())?;
  file.write_all(&1u16.to_le_bytes())?;
  file.write_all(&rate.to_le_bytes())?;
  file.write_all(&(rate * 2).to_le_bytes())?;
  file.write_all(&2u16.to_le_bytes())?;
  file.write_all(&16u16.to_le_bytes())?;
  file.write_all(b"data")?;
  file.write_all(&data_bytes.to_le_bytes())?;
  for sample in samples {
    file.write_all(&((sample.clamp(-1.0, 1.0) * f32::from(i16::MAX)) as i16).to_le_bytes())?;
  }
  file.flush()
}

/// Reads the 16-bit mono PCM the app saves, whose header is always 44 bytes.
pub fn read(path: &Path) -> std::io::Result<(Vec<f32>, u32)> {
  let data = std::fs::read(path)?;
  let Some((header, body)) = data.split_first_chunk::<44>() else { return Err(ErrorKind::InvalidData.into()) };
  let rate = u32::from_le_bytes([header[24], header[25], header[26], header[27]]);
  let mono = header[20..24] == [1, 0, 1, 0] && header[34..36] == [16, 0];
  if !header.starts_with(b"RIFF") || &header[36..40] != b"data" || !mono || rate == 0 || body.len() < 2 {
    return Err(ErrorKind::InvalidData.into());
  }
  let samples = body.as_chunks::<2>().0.iter().map(|bytes| f32::from(i16::from_le_bytes(*bytes)) / f32::from(i16::MAX));
  Ok((samples.collect(), rate))
}

/// Loudest sample in each of `count` equal slices, for drawing a waveform.
pub fn peaks(samples: &[f32], count: usize) -> Vec<f32> {
  let size = samples.len().div_ceil(count).max(1);
  samples.chunks(size).map(|slice| slice.iter().fold(0.0f32, |peak, sample| peak.max(sample.abs()))).collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn writes_a_valid_header() {
    let path = std::env::temp_dir().join("voxmelody-wav-é.wav");
    write(&path, &[0.0, 1.0, -2.0], 24_000).unwrap();
    let data = std::fs::read(&path).unwrap();
    std::fs::remove_file(&path).unwrap();
    assert_eq!(data.len(), 50);
    assert_eq!(&data[4..8], &42u32.to_le_bytes());
    assert_eq!(&data[48..50], &(-i16::MAX).to_le_bytes());
  }

  #[test]
  fn reads_what_it_writes() {
    let path = std::env::temp_dir().join("voxmelody-read-é.wav");
    write(&path, &[0.0, 1.0, -0.5], 24_000).unwrap();
    let (samples, rate) = read(&path).unwrap();
    assert_eq!((samples.len(), rate), (3, 24_000));
    assert!((samples[2] + 0.5).abs() < 0.001);
    let data = std::fs::read(&path).unwrap();
    for length in [0, 43, 44] {
      std::fs::write(&path, &data[..length]).unwrap();
      assert!(read(&path).is_err(), "{length}");
    }
    std::fs::remove_file(&path).unwrap();
  }

  #[test]
  fn limits_peaks() {
    assert!(peaks(&vec![0.5; 1001], 160).len() <= 160);
    assert_eq!(peaks(&[0.1, -0.9], 160), vec![0.1, 0.9]);
  }
}
