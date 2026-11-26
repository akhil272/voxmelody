//! Raises audio to the target loudness without clipping.

use std::collections::VecDeque;

use super::{CHANNELS, frames, from_db, loudness, peak};

pub const TARGET_LUFS: f64 = -14.0;
/// Leaves 1 dB of headroom for players and encoders.
pub const CEILING: f32 = 0.891;
const LOOKAHEAD_SECONDS: f32 = 0.005;
const RELEASE_SECONDS: f32 = 0.1;
/// More reduction than this is audible as distortion.
const MAX_REDUCTION_DB: f32 = 12.0;
const TOLERANCE_LU: f64 = 0.05;
const PASSES: usize = 8;

/// The gain for each frame that keeps every sample at or below the ceiling.
fn curve(samples: &[f32], gain: f32, rate: u32) -> Vec<f32> {
  let window = frames(LOOKAHEAD_SECONDS, rate).max(1);
  let release = 1.0 - (-1.0 / (RELEASE_SECONDS * rate as f32)).exp();
  let needed = |frame: &[f32; CHANNELS]| (CEILING / (peak(frame) * gain).max(f32::MIN_POSITIVE)).min(1.0);
  let mut gains: Vec<f32> = samples.as_chunks::<CHANNELS>().0.iter().map(needed).collect();

  // Each frame takes the lowest gain needed within the lookahead.
  let mut lowest: VecDeque<(usize, f32)> = VecDeque::new();
  for index in (0..gains.len()).rev() {
    while lowest.back().is_some_and(|(_, value)| *value >= gains[index]) {
      lowest.pop_back();
    }
    lowest.push_back((index, gains[index]));
    while lowest.front().is_some_and(|(start, _)| *start > index + window) {
      lowest.pop_front();
    }
    if let Some((_, value)) = lowest.front() {
      gains[index] = *value;
    }
  }

  // Recover slowly, then average over the lookahead so gain never jumps.
  let mut current = 1.0f32;
  let mut recent = VecDeque::with_capacity(window);
  let mut sum = 0.0f64;
  for value in &mut gains {
    current = value.min(current + (1.0 - current) * release);
    if recent.len() == window {
      sum -= f64::from(recent.pop_front().unwrap_or(0.0));
    }
    recent.push_back(current);
    sum += f64::from(current);
    *value = (sum / recent.len() as f64) as f32 * gain;
  }
  gains
}

/// Limiting lowers loudness, so the gain is corrected until the result is on target.
pub fn normalise(samples: &mut [f32], rate: u32) {
  let Some(current) = loudness::measure(samples, rate) else { return };
  let highest = from_db(MAX_REDUCTION_DB) * CEILING / peak(samples).max(f32::MIN_POSITIVE);
  let mut gain = from_db((TARGET_LUFS - current) as f32).min(highest);
  let mut gains = curve(samples, gain, rate);
  for _ in 0..PASSES {
    let limited = samples.as_chunks::<CHANNELS>().0.iter().zip(&gains).map(|(frame, gain)| frame.map(|x| x * gain));
    let Some(result) = loudness::measure_frames(limited, gains.len(), rate) else { break };
    let next = (gain * from_db((TARGET_LUFS - result) as f32)).min(highest);
    if (TARGET_LUFS - result).abs() < TOLERANCE_LU || next == gain {
      break;
    }
    gain = next;
    gains = curve(samples, gain, rate);
  }
  for (frame, gain) in samples.as_chunks_mut::<CHANNELS>().0.iter_mut().zip(gains) {
    frame.iter_mut().for_each(|sample| *sample = (*sample * gain).clamp(-CEILING, CEILING));
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  const RATE: u32 = 48_000;

  #[test]
  fn peaky_audio_reaches_the_target() {
    // A quiet tone with loud bursts, like percussion.
    let mut samples: Vec<f32> = (0..RATE as usize * 10)
      .flat_map(|i| {
        let amplitude = if i % 24_000 < 1_000 { 0.9 } else { 0.1 };
        [amplitude * (i as f32 * 0.06).sin(); CHANNELS]
      })
      .collect();
    let raised = curve(&samples, 4.0, RATE);
    for (frame, gain) in samples.as_chunks::<CHANNELS>().0.iter().zip(raised) {
      assert!(peak(frame) * gain <= CEILING + 0.001);
    }
    normalise(&mut samples, RATE);
    assert!(peak(&samples) <= CEILING);
    assert!((loudness::measure(&samples, RATE).unwrap() - TARGET_LUFS).abs() < 0.1);
  }

  #[test]
  fn leaves_silence_alone() {
    let mut samples = vec![0.0; RATE as usize * CHANNELS];
    normalise(&mut samples, RATE);
    assert!(samples.iter().all(|sample| *sample == 0.0));
    normalise(&mut [], RATE);
  }
}
