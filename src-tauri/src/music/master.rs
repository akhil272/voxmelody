//! Turns raw engine audio into a finished track of the exact length.

use crate::audio::{CHANNELS, loudness};

const TARGET_LUFS: f64 = -14.0;
/// Leaves 1 dB of headroom for players and encoders.
const CEILING: f32 = 0.891;
/// 50 dB below the loudest sample counts as silence.
const SILENCE_RATIO: f32 = 0.003;
const WINDOW_SECONDS: f32 = 0.05;
const FADE_IN_SECONDS: f32 = 0.01;
const FADE_OUT_SECONDS: f32 = 2.0;
const CROSSFADE_SECONDS: f32 = 1.0;

fn frames(seconds: f32, rate: u32) -> usize {
  (seconds * rate as f32) as usize
}

fn peak(samples: &[f32]) -> f32 {
  samples.iter().fold(0.0, |peak, sample| peak.max(sample.abs()))
}

/// The part between the first and last sound.
fn trim_silence(samples: &[f32], rate: u32) -> &[f32] {
  let threshold = peak(samples) * SILENCE_RATIO;
  let window = frames(WINDOW_SECONDS, rate).max(1) * CHANNELS;
  let audible = |slice: &[f32]| {
    let power = slice.iter().map(|sample| sample * sample).sum::<f32>() / slice.len() as f32;
    power.sqrt() > threshold
  };
  let mut heard = samples.chunks(window).enumerate().filter(|(_, slice)| audible(slice)).map(|(index, _)| index);
  let Some(first) = heard.next() else { return &[] };
  let last = heard.next_back().unwrap_or(first);
  &samples[first * window..((last + 1) * window).min(samples.len())]
}

/// True when the sound lasts for the whole track.
pub fn is_long_enough(raw: &[f32], rate: u32, seconds: u32) -> bool {
  trim_silence(raw, rate).len() >= seconds as usize * rate as usize * CHANNELS
}

/// Repeats the track with equal-power crossfades until it is long enough.
fn extend(samples: &[f32], rate: u32, length: usize) -> Vec<f32> {
  let mut out = samples.to_vec();
  let overlap = frames(CROSSFADE_SECONDS, rate).min(samples.len() / CHANNELS / 4);
  while out.len() < length {
    let start = out.len() - overlap * CHANNELS;
    for frame in 0..overlap {
      let angle = std::f32::consts::FRAC_PI_2 * frame as f32 / overlap as f32;
      for index in frame * CHANNELS..(frame + 1) * CHANNELS {
        out[start + index] = out[start + index] * angle.cos() + samples[index] * angle.sin();
      }
    }
    out.extend_from_slice(&samples[overlap * CHANNELS..]);
  }
  out
}

fn fade(samples: &mut [f32], rate: u32) {
  let total = samples.len() / CHANNELS;
  let fade_in = frames(FADE_IN_SECONDS, rate).max(1);
  let fade_out = frames(FADE_OUT_SECONDS, rate).min(total / 8).max(1);
  for (index, frame) in samples.as_chunks_mut::<CHANNELS>().0.iter_mut().enumerate() {
    let start = (index as f32 / fade_in as f32).min(1.0);
    let end = ((total - 1 - index) as f32 / fade_out as f32).min(1.0);
    // A sine curve keeps the ending audible for most of the fade.
    let gain = start * (end * std::f32::consts::FRAC_PI_2).sin();
    frame.iter_mut().for_each(|sample| *sample *= gain);
  }
}

fn normalise(samples: &mut [f32], rate: u32) {
  let Some(current) = loudness::measure(samples, rate) else { return };
  let wanted = 10f64.powf((TARGET_LUFS - current) / 20.0) as f32;
  let gain = wanted.min(CEILING / peak(samples).max(f32::MIN_POSITIVE));
  samples.iter_mut().for_each(|sample| *sample *= gain);
}

/// `None` when the engine produced no sound.
pub fn finish(raw: &[f32], rate: u32, seconds: u32) -> Option<Vec<f32>> {
  let sound = trim_silence(raw, rate);
  if sound.len() < rate as usize * CHANNELS {
    return None;
  }
  let length = seconds as usize * rate as usize * CHANNELS;
  let mut track = extend(sound, rate, length);
  track.truncate(length);
  fade(&mut track, rate);
  normalise(&mut track, rate);
  Some(track)
}

#[cfg(test)]
mod tests {
  use super::*;

  const RATE: u32 = 48_000;

  fn tone(seconds: f32, amplitude: f32) -> Vec<f32> {
    (0..frames(seconds, RATE))
      .flat_map(|i| [amplitude * (i as f32 * 440.0 * std::f32::consts::TAU / RATE as f32).sin(); CHANNELS])
      .collect()
  }

  fn silence(seconds: f32) -> Vec<f32> {
    vec![0.0; frames(seconds, RATE) * CHANNELS]
  }

  fn expected(seconds: u32) -> usize {
    (seconds * RATE) as usize * CHANNELS
  }

  #[test]
  fn cuts_to_the_exact_length_and_loudness() {
    let raw = [silence(0.5), tone(12.0, 0.9)].concat();
    assert!(is_long_enough(&raw, RATE, 10));
    let track = finish(&raw, RATE, 10).unwrap();
    assert_eq!(track.len(), expected(10));
    assert!(peak(&track) <= CEILING + 0.001);
    assert!((loudness::measure(&track, RATE).unwrap() - TARGET_LUFS).abs() < 0.3);
    assert_eq!(track[track.len() - 1], 0.0);
  }

  #[test]
  fn never_leaves_a_silent_tail() {
    let raw = [tone(6.0, 0.5), silence(6.0)].concat();
    assert!(!is_long_enough(&raw, RATE, 10));
    let track = finish(&raw, RATE, 10).unwrap();
    assert_eq!(track.len(), expected(10));
    for second in track.chunks(RATE as usize * CHANNELS).take(9) {
      assert!(peak(second) > 0.05);
    }
  }

  #[test]
  fn keeps_peaks_below_the_ceiling() {
    // Quiet music with one loud hit must not clip when raised.
    let mut raw = tone(12.0, 0.02);
    raw[RATE as usize] = 1.0;
    assert!(peak(&finish(&raw, RATE, 10).unwrap()) <= CEILING + 0.001);
  }

  #[test]
  fn rejects_silence() {
    assert!(finish(&silence(5.0), RATE, 5).is_none());
    assert!(finish(&[], RATE, 5).is_none());
  }
}
