//! Band-limited sample rate conversion.

use super::CHANNELS;

/// Zero crossings on each side of the filter.
const HALF_WIDTH: usize = 24;
/// Leaves room for the filter to roll off below the Nyquist limit.
const CUTOFF: f64 = 0.95;

fn gcd(a: usize, b: usize) -> usize {
  if b == 0 { a } else { gcd(b, a % b) }
}

fn sinc(x: f64) -> f64 {
  let x = x * std::f64::consts::PI;
  if x.abs() < 1e-9 { 1.0 } else { x.sin() / x }
}

/// Blackman window over -1..1.
fn window(x: f64) -> f64 {
  if x.abs() >= 1.0 {
    return 0.0;
  }
  let x = x * std::f64::consts::PI;
  0.42 + 0.5 * x.cos() + 0.08 * (2.0 * x).cos()
}

/// One filter per fractional position; each sums to one.
fn filters(phases: usize, half: usize, cutoff: f64) -> Vec<Vec<f32>> {
  (0..phases)
    .map(|phase| {
      let offset = phase as f64 / phases as f64;
      let taps: Vec<f64> = (0..half * 2)
        .map(|tap| {
          let x = tap as f64 - (half as f64 - 1.0) - offset;
          sinc(x * cutoff) * window(x / half as f64)
        })
        .collect();
      let sum: f64 = taps.iter().sum();
      taps.into_iter().map(|tap| (tap / sum) as f32).collect()
    })
    .collect()
}

/// Both rates must be above zero.
pub fn convert(samples: Vec<f32>, from: u32, to: u32) -> Vec<f32> {
  if from == to {
    return samples;
  }
  let divisor = gcd(from as usize, to as usize);
  let (step, phases) = (from as usize / divisor, to as usize / divisor);
  let scale = (phases as f64 / step as f64).min(1.0);
  let half = (HALF_WIDTH as f64 / scale).ceil() as usize;
  let filters = filters(phases, half, CUTOFF * scale);
  let input = samples.as_chunks::<CHANNELS>().0;
  let length = input.len() * phases / step;
  let mut out = Vec::with_capacity(length * CHANNELS);
  for index in 0..length {
    let position = index * step;
    let (base, filter) = (position / phases, &filters[position % phases]);
    let mut frame = [0.0f32; CHANNELS];
    for (tap, weight) in filter.iter().enumerate() {
      let Some(source) = (base + tap + 1).checked_sub(half).and_then(|source| input.get(source)) else { continue };
      for (sum, sample) in frame.iter_mut().zip(source) {
        *sum += sample * weight;
      }
    }
    out.extend(frame);
  }
  out
}

#[cfg(test)]
mod tests {
  use super::*;

  fn tone(rate: u32, hertz: f64) -> Vec<f32> {
    (0..rate)
      .flat_map(|i| [(0.5 * (f64::from(i) * hertz * std::f64::consts::TAU / f64::from(rate)).sin()) as f32; CHANNELS])
      .collect()
  }

  #[test]
  fn keeps_the_tone() {
    for rate in [22_050, 24_000, 44_100, 96_000] {
      let out = convert(tone(rate, 1000.0), rate, 48_000);
      let expected = tone(48_000, 1000.0);
      assert_eq!(out.len(), expected.len());
      // The ends have no neighbours to filter with.
      for index in 1_000..out.len() - 1_000 {
        assert!((out[index] - expected[index]).abs() < 0.002, "{rate} {index}");
      }
    }
  }

  #[test]
  fn removes_what_the_target_cannot_hold() {
    let out = convert(tone(96_000, 40_000.0), 96_000, 48_000);
    assert!(crate::audio::peak(&out[2_000..out.len() - 2_000]) < 0.002);
    assert!(convert(vec![], 24_000, 48_000).is_empty());
  }
}
