//! Prepares a recording of a voice for cloning.

use super::error::CloneError;
use crate::audio::{frames, from_db, peak};

/// What the page sends and the model expects.
pub const RATE: u32 = 24_000;
/// Longer recordings are cut here.
const MAX_SECONDS: f32 = 15.0;
const MIN_SECONDS: f32 = 5.0;
const WINDOW_SECONDS: f32 = 0.02;
const EDGE_SECONDS: f32 = 0.1;
/// Quiet long enough to be a gap between phrases.
const PAUSE_SECONDS: f32 = 0.12;
/// Without silence around the sample, the model runs its end into new speech.
const LEAD_SECONDS: f32 = 0.5;
const TAIL_SECONDS: f32 = 0.3;
const FADE_SECONDS: f32 = 0.02;
/// A voice quieter than this has too much noise to copy.
const MIN_LEVEL: f32 = -45.0;
/// Below the loudest moment, where speech has ended.
const SILENCE_DROP: f32 = -35.0;
const TARGET_LEVEL: f32 = -20.0;
const PEAK_LIMIT: f32 = -1.0;

fn level(samples: &[f32]) -> f32 {
  let power: f32 = samples.iter().map(|sample| sample * sample).sum();
  (power / samples.len().max(1) as f32).sqrt()
}

/// Reads little-endian floats.
pub fn decode(data: &[u8]) -> Result<Vec<f32>, CloneError> {
  let (values, rest) = data.as_chunks::<4>();
  let samples: Vec<f32> = values.iter().map(|bytes| f32::from_le_bytes(*bytes)).collect();
  if rest.is_empty() && samples.iter().all(|sample| sample.is_finite()) { Ok(samples) } else { Err(CloneError::Audio) }
}

/// Trims silence, limits the length and sets an even level.
pub fn prepare(samples: &[f32]) -> Result<Vec<f32>, CloneError> {
  let window = frames(WINDOW_SECONDS, RATE);
  let levels: Vec<f32> = samples.chunks(window).map(level).collect();
  let loudest = peak(&levels);
  if loudest < from_db(MIN_LEVEL) {
    return Err(CloneError::TooQuiet);
  }
  let heard = |level: &f32| *level > loudest * from_db(SILENCE_DROP);
  let (Some(first), Some(last)) = (levels.iter().position(heard), levels.iter().rposition(heard)) else {
    return Err(CloneError::TooQuiet);
  };
  if (last + 1 - first) * window < frames(MIN_SECONDS, RATE) {
    return Err(CloneError::TooShort);
  }
  let last = finish(&levels, first, last, heard);
  let edge = frames(EDGE_SECONDS, RATE);
  let start = (first * window).saturating_sub(edge);
  let end = ((last + 1) * window + edge).min(samples.len());
  let voice = &samples[start..end];
  let gain = (from_db(TARGET_LEVEL) / level(voice)).min(from_db(PEAK_LIMIT) / peak(voice));
  let fade = frames(FADE_SECONDS, RATE);
  let ramp = |index: usize| (index as f32 / fade as f32).min(1.0);
  let mut prepared = vec![0.0; frames(LEAD_SECONDS, RATE)];
  prepared.extend(
    voice.iter().enumerate().map(|(index, sample)| sample * gain * ramp(index) * ramp(voice.len() - 1 - index)),
  );
  prepared.resize(prepared.len() + frames(TAIL_SECONDS, RATE), 0.0);
  Ok(prepared)
}

/// The last window to keep. Speech that is cut off ends at its last pause instead.
fn finish(levels: &[f32], first: usize, last: usize, heard: impl Fn(&f32) -> bool) -> usize {
  let windows = |seconds: f32| (seconds / WINDOW_SECONDS).round() as usize;
  let limit = first + windows(MAX_SECONDS) - 1;
  if last <= limit && last + 1 < levels.len() {
    return last;
  }
  let last = last.min(limit);
  let pause = windows(PAUSE_SECONDS);
  let earliest = first + windows(MIN_SECONDS);
  let is_pause = |end: &usize| heard(&levels[*end]) && !levels[end + 1..=end + pause].iter().any(&heard);
  (earliest..last.saturating_sub(pause)).rev().find(is_pause).unwrap_or(last)
}

#[cfg(test)]
mod tests {
  use super::*;

  fn tone(seconds: f32, volume: f32) -> Vec<f32> {
    (0..frames(seconds, RATE)).map(|index| (index as f32 * 0.05).sin() * volume).collect()
  }

  fn seconds(samples: &[f32]) -> f32 {
    samples.len() as f32 / RATE as f32
  }

  #[test]
  fn trims_and_levels() {
    let silence = vec![0.0; frames(2.0, RATE)];
    for volume in [0.05, 0.9] {
      let sample = prepare(&[silence.clone(), tone(8.0, volume), silence.clone()].concat()).unwrap();
      assert!((8.8..9.1).contains(&seconds(&sample)), "{}", seconds(&sample));
      let (lead, tail) = (frames(LEAD_SECONDS, RATE), frames(TAIL_SECONDS, RATE));
      assert!((level(&sample[lead..sample.len() - tail]) - from_db(TARGET_LEVEL)).abs() < 0.005);
      assert_eq!(peak(&sample[..lead]), 0.0);
    }
  }

  #[test]
  fn limits_the_length() {
    let length = seconds(&prepare(&tone(60.0, 0.5)).unwrap());
    assert!((15.7..16.0).contains(&length), "{length}");
    assert_eq!(prepare(&tone(4.0, 0.5)), Err(CloneError::TooShort));
    assert_eq!(prepare(&[tone(1.0, 0.5), vec![0.0; frames(9.0, RATE)]].concat()), Err(CloneError::TooShort));
  }

  #[test]
  fn cut_off_speech_ends_at_a_pause() {
    let gap = vec![0.0; frames(0.3, RATE)];
    for rest in [4.0, 30.0] {
      let length = seconds(&prepare(&[tone(8.0, 0.5), gap.clone(), tone(rest, 0.5)].concat()).unwrap());
      assert!((8.7..9.1).contains(&length), "{length}");
    }
    // A pause that leaves too little is ignored.
    let length = seconds(&prepare(&[tone(2.0, 0.5), gap, tone(6.0, 0.5)].concat()).unwrap());
    assert!((9.0..9.2).contains(&length), "{length}");
  }

  #[test]
  fn refuses_silence_and_noise() {
    assert_eq!(prepare(&[]), Err(CloneError::TooQuiet));
    assert_eq!(prepare(&tone(8.0, 0.0)), Err(CloneError::TooQuiet));
    assert_eq!(prepare(&tone(8.0, 0.005)), Err(CloneError::TooQuiet));
  }

  #[test]
  fn peaks_stay_below_the_limit() {
    let mut samples = tone(8.0, 0.02);
    samples[100_000] = 1.0;
    assert!(peak(&prepare(&samples).unwrap()) <= from_db(PEAK_LIMIT) + 0.001);
  }

  #[test]
  fn decodes_only_whole_numbers() {
    assert_eq!(decode(&0.5f32.to_le_bytes()).unwrap(), [0.5]);
    assert!(decode(&[0, 0, 0]).is_err());
    assert!(decode(&f32::NAN.to_le_bytes()).is_err());
  }
}
