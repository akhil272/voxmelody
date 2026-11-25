use serde::Deserialize;
use tauri::{AppHandle, Emitter, Manager, State};

use super::engine::{Request, VoiceEngine};
use super::error::VoiceError;
use super::voices::{self, Voice};
use crate::library::commands as library;
use crate::library::store::Kind;
use crate::models::manager::ModelManager;
use crate::models::manifest::{self, PackKind};
use crate::takes::{Take, Takes};

const PROGRESS_EVENT: &str = "voice://progress";
const MAX_CHARACTERS: usize = 10_000;
const PREVIEW_CHARACTERS: usize = 160;
const SPEED: std::ops::RangeInclusive<f32> = 0.5..=1.5;
const PAUSE: std::ops::RangeInclusive<f32> = 1.0..=5.0;
const MAX_THREADS: u32 = 64;
const MAX_STEPS: u32 = 32;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpeechRequest {
  text: String,
  model: String,
  voice: String,
  speed: f32,
  preview: bool,
  /// Length of the pauses between sentences, where 1 is normal.
  pause: f32,
  /// Zero lets the engine choose.
  threads: u32,
  /// Zero uses the model's own number.
  steps: u32,
}

impl SpeechRequest {
  fn is_valid(&self) -> bool {
    SPEED.contains(&self.speed) && PAUSE.contains(&self.pause) && self.threads <= MAX_THREADS && self.steps <= MAX_STEPS
  }
}

pub fn setup(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
  app.manage(VoiceEngine::new(crate::engine::path("voxmelody-voice")?));
  Ok(())
}

/// The opening of a script, cut at a sentence or word where possible.
fn preview_text(text: &str) -> &str {
  let text = text.trim();
  let Some((limit, _)) = text.char_indices().nth(PREVIEW_CHARACTERS) else { return text };
  let head = &text[..limit];
  let end = head
    .rfind(['.', '!', '?', '\n', '।'])
    .map(|index| index + head[index..].chars().next().map_or(1, char::len_utf8))
    .or_else(|| head.rfind(char::is_whitespace))
    .unwrap_or(limit);
  &head[..end]
}

#[tauri::command]
pub fn list_voices() -> Vec<Voice> {
  manifest::packs().iter().flat_map(|pack| voices::of(pack)).collect()
}

#[tauri::command]
pub async fn speak(
  app: AppHandle,
  engine: State<'_, VoiceEngine>,
  models: State<'_, ModelManager>,
  takes: State<'_, Takes>,
  request: SpeechRequest,
) -> Result<Take, VoiceError> {
  let model = manifest::find(&request.model).ok_or(VoiceError::Request)?;
  let voice = voices::find(model, &request.voice).filter(|_| request.is_valid()).ok_or(VoiceError::Request)?;
  let engine_model = model.voice.as_ref().ok_or(VoiceError::Request)?;
  if request.text.chars().count() > MAX_CHARACTERS {
    return Err(VoiceError::TooLong);
  }
  let preview = request.preview;
  let text = if preview { preview_text(&request.text) } else { request.text.as_str() };
  let (id, output) = takes.reserve();
  let locate = async {
    models.installed(&model.id, PackKind::Voice).await.map(|(dir, _)| dir).ok_or(VoiceError::NotInstalled)
  };
  let request = Request {
    family: &engine_model.family,
    model: &engine_model.file,
    text,
    language: &voice.language,
    speaker: voice.speaker,
    speed: request.speed,
    pause: request.pause,
    threads: request.threads,
    steps: request.steps,
    output: &output,
  };
  let report = |value: f32| {
    let _ = app.emit(PROGRESS_EVENT, value);
  };
  match engine.speak(locate, &request, report).await {
    Ok(speech) if preview => {
      takes.keep(id);
      Ok(Take { id, duration_ms: speech.duration_ms, peaks: speech.peaks })
    }
    Ok(speech) => library::keep(&app, Kind::Voice, text, output.clone(), speech.duration_ms, speech.peaks).await.map_err(|_| {
      takes.discard(id);
      VoiceError::Output
    }),
    Err(error) => {
      takes.discard(id);
      Err(error)
    }
  }
}

#[tauri::command]
pub fn cancel_speech(engine: State<'_, VoiceEngine>) {
  engine.cancel();
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn previews_whole_sentences() {
    assert_eq!(preview_text("  Short one.  "), "Short one.");
    let text = format!("First sentence. {}", "word ".repeat(60));
    assert_eq!(preview_text(&text), "First sentence.");
    let words = "word ".repeat(60);
    assert!(preview_text(&words).ends_with("word"));
    let accents = "é".repeat(400);
    assert_eq!(preview_text(&accents).chars().count(), PREVIEW_CHARACTERS);
    let hindi = format!("नमस्ते। {}", "शब्द ".repeat(60));
    assert_eq!(preview_text(&hindi), "नमस्ते।");
  }
}
