use serde::Deserialize;
use sherpa_onnx::{
  GenerationConfig, OfflineTts, OfflineTtsConfig, OfflineTtsKittenModelConfig, OfflineTtsKokoroModelConfig,
  OfflineTtsModelConfig, OfflineTtsSupertonicModelConfig,
};

use crate::{ErrorCode, Event, Request, emit, text, wav};

const PEAK_COUNT: usize = 160;
const MAX_THREADS: usize = 4;
/// Quieter than this is treated as silence.
const SILENCE: f32 = 0.001;

/// Pauses at normal length, in the units of each family.
const PAUSE_SCALE: f32 = 0.2;
const PAUSE_SECONDS: f32 = 0.3;
const ESPEAK: &str = "espeak-ng-data";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Family {
  Kokoro,
  Kitten,
  Supertonic,
}

/// Everything fixed when a model loads.
#[derive(PartialEq, Eq)]
struct Key {
  family: Family,
  model: String,
  language: String,
  threads: usize,
}

impl Key {
  fn of(request: &Request) -> Self {
    let automatic = || std::thread::available_parallelism().map_or(1, |count| (count.get() / 2).clamp(1, MAX_THREADS));
    // Only Kokoro loads differently for each language.
    let language = if request.family == Family::Kokoro { request.language.clone() } else { String::new() };
    let threads = if request.threads == 0 { automatic() } else { request.threads };
    Self { family: request.family, model: request.model.clone(), language, threads }
  }

  fn config(&self) -> OfflineTtsModelConfig {
    let mut config = OfflineTtsModelConfig { num_threads: self.threads as i32, ..Default::default() };
    let (model, voices) = (Some(self.model.clone()), Some("voices.bin".into()));
    let (tokens, data_dir) = (Some("tokens.txt".into()), Some(ESPEAK.into()));
    match self.family {
      Family::Kokoro => {
        let english = text::is_english(&self.language);
        let lexicon = if self.language == "en-gb" { "lexicon-gb-en.txt" } else { "lexicon-us-en.txt" };
        let lang = Some(if english { String::new() } else { self.language.clone() });
        let lexicon = Some(lexicon.into());
        config.kokoro =
          OfflineTtsKokoroModelConfig { model, voices, tokens, data_dir, lexicon, lang, ..Default::default() };
      }
      Family::Kitten => {
        config.kitten = OfflineTtsKittenModelConfig { model, voices, tokens, data_dir, ..Default::default() };
      }
      Family::Supertonic => {
        config.supertonic = OfflineTtsSupertonicModelConfig {
          duration_predictor: Some("duration_predictor.int8.onnx".into()),
          text_encoder: Some("text_encoder.int8.onnx".into()),
          vector_estimator: Some("vector_estimator.int8.onnx".into()),
          vocoder: Some("vocoder.int8.onnx".into()),
          tts_json: Some("tts.json".into()),
          unicode_indexer: Some("unicode_indexer.bin".into()),
          voice_style: Some("voice.bin".into()),
        };
      }
    }
    config
  }
}

fn generation(request: &Request) -> GenerationConfig {
  let mut config =
    GenerationConfig { sid: request.speaker, speed: request.speed.clamp(0.5, 2.0), ..Default::default() };
  if request.family == Family::Supertonic {
    let pause = serde_json::json!(request.pause * PAUSE_SECONDS);
    config.extra = Some([("lang".into(), request.language.clone().into()), ("silence_duration".into(), pause)].into());
    if request.steps > 0 {
      config.num_steps = request.steps;
    }
  } else {
    config.silence_scale = request.pause * PAUSE_SCALE;
  }
  config
}

#[derive(Default)]
pub struct Engine {
  loaded: Option<(Key, OfflineTts)>,
}

impl Engine {
  fn load(&mut self, key: Key) -> Result<&OfflineTts, ErrorCode> {
    if self.loaded.as_ref().is_none_or(|(current, _)| *current != key) {
      self.loaded = None;
      let config = OfflineTtsConfig { model: key.config(), max_num_sentences: 1, ..Default::default() };
      let tts = OfflineTts::create(&config).ok_or(ErrorCode::Model)?;
      self.loaded = Some((key, tts));
    }
    self.loaded.as_ref().map(|(_, tts)| tts).ok_or(ErrorCode::Model)
  }

  pub fn speak(&mut self, request: &Request) -> Result<Event, ErrorCode> {
    let mut text = text::prepare(&request.text, &request.language);
    if request.family == Family::Supertonic && text::is_english(&request.language) {
      text = text::spell_prices(&text);
    }
    if !text.chars().any(char::is_alphanumeric) {
      return Err(ErrorCode::NoSpeech);
    }
    let tts = self.load(Key::of(request))?;
    if !(0..tts.num_speakers()).contains(&request.speaker) {
      return Err(ErrorCode::Request);
    }
    let audio = tts
      .generate_with_config(
        &text,
        &generation(request),
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
