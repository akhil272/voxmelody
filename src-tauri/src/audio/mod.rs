pub mod limiter;
pub mod loudness;
pub mod resample;
pub mod shape;
pub mod wav;

/// Interleaved stereo.
pub const CHANNELS: usize = 2;

pub fn frames(seconds: f32, rate: u32) -> usize {
  (seconds * rate as f32) as usize
}

pub fn peak(samples: &[f32]) -> f32 {
  samples.iter().fold(0.0, |peak, sample| peak.max(sample.abs()))
}

pub fn from_db(db: f32) -> f32 {
  10f32.powf(db / 20.0)
}
