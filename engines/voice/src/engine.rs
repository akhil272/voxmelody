use serde::Deserialize;
use sherpa_onnx::{
  GenerationConfig, OfflineTts, OfflineTtsConfig, OfflineTtsKokoroModelConfig, OfflineTtsModelConfig,
};

use crate::{ErrorCode, Event, Request, emit, text, wav};

const PEAK_COUNT: usize = 160;
const MAX_THREADS: usize = 4;
/// Quieter than this is treated as silence.
const SILENCE: f32 = 0.001;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum Language {
  #[serde(rename = "en-us")]
  EnUs,
  #[serde(rename = "en-gb")]
  EnGb,
  #[serde(rename = "es")]
  Es,
  #[serde(rename = "fr")]
  Fr,
  #[serde(rename = "hi")]
  Hi,
  #[serde(rename = "it")]
  It,
  #[serde(rename = "pt-br")]
  PtBr,
}

impl Language {
  fn code(self) -> &'static str {
    match self {
      Self::EnUs => "en-us",
      Self::EnGb => "en-gb",
      Self::Es => "es",
      Self::Fr => "fr",
      Self::Hi => "hi",
      Self::It => "it",
      Self::PtBr => "pt-br",
    }
  }

  pub fn is_english(self) -> bool {
    matches!(self, Self::EnUs | Self::EnGb)
  }

  fn lexicon(self) -> &'static str {
    if self == Self::EnGb { "lexicon-gb-en.txt" } else { "lexicon-us-en.txt" }
  }
}

#[derive(Default)]
pub struct Engine {
  loaded: Option<(Language, OfflineTts)>,
}

impl Engine {
  /// Language is fixed when the model loads, so switching reloads it.
  fn load(&mut self, language: Language) -> Result<&OfflineTts, ErrorCode> {
    if self.loaded.as_ref().is_none_or(|(current, _)| *current != language) {
      self.loaded = None;
      let threads = std::thread::available_parallelism().map_or(1, |count| (count.get() / 2).clamp(1, MAX_THREADS));
      let tts = OfflineTts::create(&OfflineTtsConfig {
        model: OfflineTtsModelConfig {
          kokoro: OfflineTtsKokoroModelConfig {
            model: Some("model.onnx".into()),
            voices: Some("voices.bin".into()),
            tokens: Some("tokens.txt".into()),
            data_dir: Some("espeak-ng-data".into()),
            lexicon: Some(language.lexicon().into()),
            lang: Some(if language.is_english() { String::new() } else { language.code().into() }),
            ..Default::default()
          },
          num_threads: threads as i32,
          ..Default::default()
        },
        max_num_sentences: 1,
        ..Default::default()
      })
      .ok_or(ErrorCode::Model)?;
      self.loaded = Some((language, tts));
    }
    self.loaded.as_ref().map(|(_, tts)| tts).ok_or(ErrorCode::Model)
  }

  pub fn speak(&mut self, request: &Request) -> Result<Event, ErrorCode> {
    let text = text::prepare(&request.text, request.language);
    if !text.chars().any(char::is_alphanumeric) {
      return Err(ErrorCode::NoSpeech);
    }
    let tts = self.load(request.language)?;
    if !(0..tts.num_speakers()).contains(&request.speaker) {
      return Err(ErrorCode::Request);
    }
    let config = GenerationConfig { sid: request.speaker, speed: request.speed.clamp(0.5, 2.0), ..Default::default() };
    let audio = tts
      .generate_with_config(
        &text,
        &config,
        Some(|_: &[f32], value: f32| {
          emit(&Event::Progress { value });
          true
        }),
      )
      .ok_or(ErrorCode::NoSpeech)?;
    let samples = audio.samples();
    if !samples.iter().any(|sample| sample.abs() > SILENCE) {
      return Err(ErrorCode::NoSpeech);
    }
    let rate = audio.sample_rate() as u32;
    wav::write(&request.output, samples, rate).map_err(|_| ErrorCode::Output)?;
    Ok(Event::Done {
      duration_ms: samples.len() as u64 * 1000 / u64::from(rate),
      peaks: wav::peaks(samples, PEAK_COUNT),
    })
  }
}
