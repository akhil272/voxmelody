//! Fits music to a length and fades its ends.

use super::{CHANNELS, frames};

const CROSSFADE_SECONDS: f32 = 1.0;

/// Exactly `length` samples, repeating the track with equal-power crossfades.
pub fn extend(samples: &[f32], rate: u32, length: usize) -> Vec<f32> {
  let mut out = Vec::with_capacity(length + samples.len());
  out.extend_from_slice(samples);
  let overlap = frames(CROSSFADE_SECONDS, rate).min(samples.len() / CHANNELS / 4);
  while out.len() < length && !samples.is_empty() {
    let start = out.len() - overlap * CHANNELS;
    for frame in 0..overlap {
      let angle = std::f32::consts::FRAC_PI_2 * frame as f32 / overlap as f32;
      for index in frame * CHANNELS..(frame + 1) * CHANNELS {
        out[start + index] = out[start + index] * angle.cos() + samples[index] * angle.sin();
      }
    }
    out.extend_from_slice(&samples[overlap * CHANNELS..]);
  }
  out.truncate(length);
  out
}

/// Both fades are in frames.
pub fn fade(samples: &mut [f32], fade_in: usize, fade_out: usize) {
  let total = samples.len() / CHANNELS;
  let (fade_in, fade_out) = (fade_in.max(1) as f32, fade_out.max(1) as f32);
  for (index, frame) in samples.as_chunks_mut::<CHANNELS>().0.iter_mut().enumerate() {
    let start = (index as f32 / fade_in).min(1.0);
    let end = ((total - 1 - index) as f32 / fade_out).min(1.0);
    // A sine curve keeps the ending audible for most of the fade.
    let gain = start * (end * std::f32::consts::FRAC_PI_2).sin();
    frame.iter_mut().for_each(|sample| *sample *= gain);
  }
}
