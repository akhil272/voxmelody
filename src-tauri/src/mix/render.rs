//! Places music under a voice and lowers it while the voice speaks.

use serde::Deserialize;

use crate::audio::wav::Audio;
use crate::audio::{CHANNELS, frames, from_db, limiter, loudness, resample, shape};
use crate::music::master;

pub const RATE: u32 = 48_000;
/// How far the music sits below speech at no volume and at full volume.
const QUIETEST_DB: f32 = 28.0;
const LOUDEST_DB: f32 = 12.0;
/// How much louder the music is when nobody speaks.
const DUCK_DB: f32 = 10.0;
/// Sound this far below the voice's loudness is a pause.
const SPEECH_GATE_DB: f32 = -25.0;
const STEP_SECONDS: f32 = 0.02;
const ATTACK_SECONDS: f32 = 0.3;
const RELEASE_SECONDS: f32 = 0.8;
/// Shorter pauses keep the music low, so it does not pump between words.
const HOLD_SECONDS: f32 = 0.6;
const FADE_IN_SECONDS: f32 = 1.0;

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
  /// Music volume from 0 to 1.
  pub volume: f32,
  /// Seconds of music before the voice.
  pub intro: f32,
  /// Seconds of music after the voice.
  pub outro: f32,
}

fn steps(seconds: f32) -> usize {
  (seconds / STEP_SECONDS).round() as usize
}

/// How far the music is ducked at each step, from 0 to 1.
fn ducking(voice: &[f32], gate: f32) -> Vec<f32> {
  let size = frames(STEP_SECONDS, RATE) * CHANNELS;
  let mut speaking = vec![false; voice.len().div_ceil(size)];
  for (index, slice) in voice.chunks(size).enumerate() {
    let power = slice.iter().map(|sample| sample * sample).sum::<f32>() / slice.len() as f32;
    if power.sqrt() > gate {
      // Starts early enough to be fully low at the first word.
      let last = (index + 1 + steps(HOLD_SECONDS)).min(speaking.len());
      speaking[index.saturating_sub(steps(ATTACK_SECONDS))..last].fill(true);
    }
  }
  let (attack, release) = (STEP_SECONDS / ATTACK_SECONDS, STEP_SECONDS / RELEASE_SECONDS);
  let mut amount = if speaking.first() == Some(&true) { 1.0f32 } else { 0.0 };
  let follow = |speaks: bool| {
    amount = if speaks { (amount + attack).min(1.0) } else { (amount - release).max(0.0) };
    amount
  };
  speaking.into_iter().map(follow).collect()
}

/// Music of the exact length. Short tracks repeat without their faded ending.
fn fit(mut music: Vec<f32>, length: usize) -> Vec<f32> {
  if music.len() >= length {
    music.truncate(length);
    return music;
  }
  let body = music.len() - master::fade_out(music.len(), RATE) * CHANNELS;
  shape::extend(&music[..body], RATE, length)
}

fn add_music(mix: &mut [f32], music: &[f32], ducked: &[f32], level: f32) {
  let size = frames(STEP_SECONDS, RATE);
  let pairs = mix.as_chunks_mut::<CHANNELS>().0.iter_mut().zip(music.as_chunks::<CHANNELS>().0);
  for (index, (frame, music)) in pairs.enumerate() {
    let (step, part) = (index / size, (index % size) as f32 / size as f32);
    let current = ducked[step];
    let next = ducked.get(step + 1).copied().unwrap_or(current);
    let gain = level * from_db(-DUCK_DB * (current + (next - current) * part));
    for (sample, music) in frame.iter_mut().zip(music) {
      *sample += music * gain;
    }
  }
}

/// `None` when either recording has no sound.
pub fn render(voice: Audio, music: Audio, settings: &Settings) -> Option<Vec<f32>> {
  let voice = resample::convert(voice.samples, voice.rate, RATE);
  let music = resample::convert(music.samples, music.rate, RATE);
  let voice_loudness = loudness::measure(&voice, RATE)? as f32;
  let music_loudness = loudness::measure(&music, RATE)? as f32;

  let intro = frames(settings.intro, RATE) * CHANNELS;
  let outro = frames(settings.outro, RATE);
  let length = intro + voice.len() + outro * CHANNELS;
  let mut mix = Vec::with_capacity(length);
  mix.resize(intro, 0.0);
  mix.extend_from_slice(&voice);
  mix.resize(length, 0.0);
  drop(voice);

  let mut music = fit(music, length);
  shape::fade(&mut music, frames(settings.intro.min(FADE_IN_SECONDS), RATE), outro);
  let under = QUIETEST_DB + (LOUDEST_DB - QUIETEST_DB) * settings.volume;
  let level = from_db(voice_loudness - music_loudness - under + DUCK_DB);
  let ducked = ducking(&mix, from_db(voice_loudness + SPEECH_GATE_DB));
  add_music(&mut mix, &music, &ducked, level);
  drop(music);
  limiter::normalise(&mut mix, RATE);
  Some(mix)
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::audio::peak;

  const SETTINGS: Settings = Settings { volume: 0.5, intro: 2.0, outro: 3.0 };

  fn tone(seconds: f32, hertz: f32, amplitude: f32, rate: u32) -> Vec<f32> {
    (0..frames(seconds, rate))
      .flat_map(|i| [amplitude * (i as f32 * hertz * std::f32::consts::TAU / rate as f32).sin(); CHANNELS])
      .collect()
  }

  /// Parts of seconds and amplitude.
  fn voice(parts: &[(f32, f32)]) -> Audio {
    let samples = parts.iter().flat_map(|(seconds, amplitude)| tone(*seconds, 1000.0, *amplitude, 24_000)).collect();
    Audio { samples, rate: 24_000 }
  }

  fn music(seconds: f32) -> Audio {
    Audio { samples: tone(seconds, 200.0, 0.4, RATE), rate: RATE }
  }

  fn at(mix: &[f32], start: f32, end: f32) -> &[f32] {
    &mix[frames(start, RATE) * CHANNELS..frames(end, RATE) * CHANNELS]
  }

  /// Strength of the 200 Hz music alone, in decibels.
  fn music_db(samples: &[f32]) -> f32 {
    let (mut sin, mut cos) = (0.0, 0.0);
    for (index, frame) in samples.as_chunks::<CHANNELS>().0.iter().enumerate() {
      let angle = index as f32 * 200.0 * std::f32::consts::TAU / RATE as f32;
      sin += frame[0] * angle.sin();
      cos += frame[0] * angle.cos();
    }
    20.0 * (sin.hypot(cos) * 2.0 / (samples.len() / CHANNELS) as f32).log10()
  }

  #[test]
  fn reaches_length_loudness_and_ceiling() {
    for seconds in [5.0, 14.0, 60.0] {
      let mix = render(voice(&[(10.0, 0.2)]), music(seconds), &SETTINGS).unwrap();
      assert_eq!(mix.len(), frames(15.0, RATE) * CHANNELS);
      assert!((loudness::measure(&mix, RATE).unwrap() - limiter::TARGET_LUFS).abs() < 0.1);
      assert!(peak(&mix) <= limiter::CEILING);
      // Music plays until the end, then fades.
      assert!(peak(at(&mix, 13.0, 14.0)) > 0.02);
      assert!(peak(&mix[mix.len() - 20..]) < 0.01);
    }
  }

  #[test]
  fn lowers_music_under_speech_and_lifts_it_in_pauses() {
    let mix = render(voice(&[(5.0, 0.3), (4.0, 0.0), (5.0, 0.3)]), music(60.0), &SETTINGS).unwrap();
    let spoken = music_db(at(&mix, 3.0, 6.0));
    for open in [at(&mix, 1.2, 1.5), at(&mix, 8.8, 10.4)] {
      let lift = music_db(open) - spoken;
      assert!((lift - DUCK_DB).abs() < 1.5, "{lift}");
    }
    // Low again by the next word.
    let start = music_db(at(&mix, 11.0, 11.5)) - spoken;
    assert!(start.abs() < 1.0, "{start}");
  }

  #[test]
  fn volume_changes_the_music_and_short_pauses_do_not() {
    let parts = [(4.0, 0.3), (0.3, 0.0), (4.0, 0.3)];
    let levels = |volume| {
      let mix = render(voice(&parts), music(60.0), &Settings { volume, ..SETTINGS }).unwrap();
      (music_db(at(&mix, 3.0, 5.0)), music_db(at(&mix, 6.0, 6.3)))
    };
    let ((quiet, _), (loud, pause)) = (levels(0.0), levels(1.0));
    assert!((loud - quiet - (QUIETEST_DB - LOUDEST_DB)).abs() < 1.0);
    assert!((pause - loud).abs() < 1.0);
  }

  #[test]
  fn refuses_silence() {
    assert!(render(voice(&[(3.0, 0.0)]), music(10.0), &SETTINGS).is_none());
    assert!(render(voice(&[(3.0, 0.3)]), Audio { samples: vec![], rate: RATE }, &SETTINGS).is_none());
  }

  #[test]
  fn repeats_very_short_music_without_an_intro() {
    let settings = Settings { volume: 1.0, intro: 0.0, outro: 1.0 };
    let mix = render(voice(&[(3.0, 0.3)]), music(1.5), &settings).unwrap();
    assert_eq!(mix.len(), frames(4.0, RATE) * CHANNELS);
    assert!(mix.iter().all(|sample| sample.is_finite()));
  }
}
