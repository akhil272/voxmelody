//! Integrated loudness after ITU-R BS.1770, with both gates.

use super::CHANNELS;

const BLOCK_SECONDS: f64 = 0.4;
/// Blocks overlap by 75%.
const STEPS_PER_BLOCK: usize = 4;
const ABSOLUTE_GATE: f64 = -70.0;
const RELATIVE_GATE: f64 = -10.0;

struct Biquad {
  b: [f64; 3],
  a: [f64; 2],
  z: [f64; 2],
}

impl Biquad {
  fn run(&mut self, x: f64) -> f64 {
    let y = self.b[0] * x + self.z[0];
    self.z[0] = self.b[1] * x - self.a[0] * y + self.z[1];
    self.z[1] = self.b[2] * x - self.a[1] * y;
    y
  }
}

/// High shelf then high pass, for any sample rate.
fn k_weighting(rate: f64) -> [Biquad; 2] {
  let (gain, q) = (3.999_843_853_973_347_f64, 0.707_175_236_955_419_6);
  let k = (std::f64::consts::PI * 1_681.974_450_955_533 / rate).tan();
  let vh = 10f64.powf(gain / 20.0);
  let vb = vh.powf(0.499_666_774_154_541_6);
  let a0 = 1.0 + k / q + k * k;
  let shelf = Biquad {
    b: [(vh + vb * k / q + k * k) / a0, 2.0 * (k * k - vh) / a0, (vh - vb * k / q + k * k) / a0],
    a: [2.0 * (k * k - 1.0) / a0, (1.0 - k / q + k * k) / a0],
    z: [0.0; 2],
  };
  let q = 0.500_327_037_323_877_3;
  let k = (std::f64::consts::PI * 38.135_470_876_024_44 / rate).tan();
  let a0 = 1.0 + k / q + k * k;
  let pass = Biquad {
    b: [1.0, -2.0, 1.0],
    a: [2.0 * (k * k - 1.0) / a0, (1.0 - k / q + k * k) / a0],
    z: [0.0; 2],
  };
  [shelf, pass]
}

fn to_lufs(power: f64) -> f64 {
  -0.691 + 10.0 * power.log10()
}

fn mean(values: &[f64]) -> f64 {
  values.iter().sum::<f64>() / values.len().max(1) as f64
}

/// Loudness in LUFS, or `None` for silence and clips shorter than one block.
pub fn measure(samples: &[f32], rate: u32) -> Option<f64> {
  let step = ((f64::from(rate) * BLOCK_SECONDS) as usize / STEPS_PER_BLOCK).max(1);
  let mut filters: [[Biquad; 2]; CHANNELS] = std::array::from_fn(|_| k_weighting(f64::from(rate)));
  let mut powers = vec![0.0; samples.len() / CHANNELS / step];
  for (index, frame) in samples.as_chunks::<CHANNELS>().0.iter().enumerate() {
    let Some(power) = powers.get_mut(index / step) else { break };
    for (sample, [shelf, pass]) in frame.iter().zip(&mut filters) {
      let weighted = pass.run(shelf.run(f64::from(*sample)));
      *power += weighted * weighted / step as f64;
    }
  }
  let blocks: Vec<f64> = powers.windows(STEPS_PER_BLOCK).map(mean).collect();
  let heard: Vec<f64> = blocks.into_iter().filter(|power| to_lufs(*power) > ABSOLUTE_GATE).collect();
  let gate = to_lufs(mean(&heard)) + RELATIVE_GATE;
  let loud: Vec<f64> = heard.into_iter().filter(|power| to_lufs(*power) > gate).collect();
  (!loud.is_empty()).then(|| to_lufs(mean(&loud)))
}

#[cfg(test)]
mod tests {
  use super::*;

  fn sine(amplitude: f64, seconds: f64, rate: u32) -> Vec<f32> {
    (0..(seconds * f64::from(rate)) as usize)
      .flat_map(|i| {
        let x = amplitude * (std::f64::consts::TAU * 1000.0 * i as f64 / f64::from(rate)).sin();
        [x as f32; CHANNELS]
      })
      .collect()
  }

  #[test]
  fn matches_the_reference_tone() {
    // EBU Tech 3341: a stereo 1 kHz sine at -23 dBFS reads -23 LUFS.
    for rate in [44_100, 48_000] {
      let loudness = measure(&sine(10f64.powf(-23.0 / 20.0), 5.0, rate), rate).unwrap();
      assert!((loudness + 23.0).abs() < 0.1, "{loudness}");
    }
  }

  #[test]
  fn ignores_silence() {
    let mut samples = sine(0.1, 5.0, 48_000);
    let before = measure(&samples, 48_000).unwrap();
    samples.extend(vec![0.0; 48_000 * CHANNELS * 10]);
    assert!((measure(&samples, 48_000).unwrap() - before).abs() < 0.3);
    assert!(measure(&vec![0.0; 96_000], 48_000).is_none());
    assert!(measure(&[], 48_000).is_none());
  }
}
