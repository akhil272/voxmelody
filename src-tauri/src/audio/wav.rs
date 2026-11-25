use std::io::{BufWriter, Error, ErrorKind, Write};
use std::path::Path;

use super::CHANNELS;

const HEADER_BYTES: u32 = 36;
const SAMPLE_BYTES: u32 = 2;
const RATES: std::ops::RangeInclusive<u32> = 8_000..=192_000;

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

/// Interleaved stereo with its sample rate.
pub struct Audio {
  pub samples: Vec<f32>,
  pub rate: u32,
}

fn invalid() -> Error {
  ErrorKind::InvalidData.into()
}

/// The name and body of each chunk in a RIFF file. A cut-off chunk ends early.
fn chunks(data: &[u8]) -> impl Iterator<Item = (&[u8], &[u8])> {
  let mut rest = data;
  std::iter::from_fn(move || {
    let (id, after) = rest.split_at_checked(4)?;
    let (size, body) = after.split_first_chunk::<4>()?;
    let size = (u32::from_le_bytes(*size) as usize).min(body.len());
    // Chunks are padded to an even size.
    rest = &body[(size + size % 2).min(body.len())..];
    Some((id, &body[..size]))
  })
}

/// Reads mono or stereo 16-bit PCM; mono plays on both sides.
pub fn read(path: &Path) -> std::io::Result<Audio> {
  let data = std::fs::read(path)?;
  if data.get(..4) != Some(b"RIFF") || data.get(8..12) != Some(b"WAVE") {
    return Err(invalid());
  }
  let find = |name: &[u8]| chunks(&data[12..]).find(|(id, _)| *id == name).map(|(_, body)| body).ok_or_else(invalid);
  let (format, body) = (find(b"fmt ")?, find(b"data")?);
  let Some(&[1, 0, channels, 0, r0, r1, r2, r3, _, _, _, _, _, _, 16, 0]) = format.first_chunk::<16>() else {
    return Err(invalid());
  };
  let rate = u32::from_le_bytes([r0, r1, r2, r3]);
  if !RATES.contains(&rate) {
    return Err(invalid());
  }
  let samples = body.as_chunks::<2>().0.iter().map(|bytes| f32::from(i16::from_le_bytes(*bytes)) / f32::from(i16::MAX));
  let samples = match channels {
    1 => samples.flat_map(|sample| [sample; CHANNELS]).collect(),
    2 => samples.collect(),
    _ => return Err(invalid()),
  };
  Ok(Audio { samples, rate })
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

  #[test]
  fn reads_what_it_writes() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("a.wav");
    write(&path, &[0.0, 1.0, -1.0, 0.5], 48_000).unwrap();
    let audio = read(&path).unwrap();
    assert_eq!((audio.rate, audio.samples.len()), (48_000, 4));
    assert!((audio.samples[3] - 0.5).abs() < 0.001);

    // Mono, with an odd-sized chunk before the audio.
    let mut data = std::fs::read(&path).unwrap();
    data[22] = 1;
    data.splice(36..36, *b"LIST\x01\0\0\0x\0");
    std::fs::write(&path, &data).unwrap();
    assert_eq!(read(&path).unwrap().samples.len(), 8);
  }

  #[test]
  fn refuses_damaged_files() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("a.wav");
    write(&path, &[0.1; 8], 48_000).unwrap();
    let data = std::fs::read(&path).unwrap();
    for length in 0..data.len() {
      std::fs::write(&path, &data[..length]).unwrap();
      assert_eq!(read(&path).is_ok(), length >= 44, "{length}");
    }
    // Wrong marker, encoding, channel count, rate and sample size.
    for (index, value) in [(0, 0), (20, 3), (22, 6), (25, 0), (34, 24)] {
      let mut broken = data.clone();
      broken[index] = value;
      std::fs::write(&path, broken).unwrap();
      assert!(read(&path).is_err(), "{index}");
    }
  }
}
