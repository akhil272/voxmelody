use serde::Deserialize;
use tauri::{AppHandle, Emitter, Manager, State};

use super::engine::{Request, VoiceEngine};
use super::error::VoiceError;
use super::voices::{self, VOICES, Voice};
use crate::models::manager::ModelManager;
use crate::takes::{Take, TakeKind, Takes};

const PROGRESS_EVENT: &str = "voice://progress";
const MAX_CHARACTERS: usize = 10_000;
const PREVIEW_CHARACTERS: usize = 160;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpeechRequest {
  text: String,
  voice: String,
  speed: f32,
  preview: bool,
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
pub fn list_voices() -> &'static [Voice] {
  &VOICES
}

#[tauri::command]
pub async fn speak(
  app: AppHandle,
  engine: State<'_, VoiceEngine>,
  models: State<'_, ModelManager>,
  takes: State<'_, Takes>,
  request: SpeechRequest,
) -> Result<Take, VoiceError> {
  let voice = voices::find(&request.voice).ok_or(VoiceError::Request)?;
  if request.text.chars().count() > MAX_CHARACTERS {
    return Err(VoiceError::TooLong);
  }
  let (text, kind) = if request.preview {
    (preview_text(&request.text), TakeKind::Preview)
  } else {
    (request.text.as_str(), TakeKind::Voice)
  };
  let (id, output) = takes.reserve();
  let locate = async { models.installed_dir("voice").await.ok_or(VoiceError::NotInstalled) };
  let request =
    Request { text, language: voice.language, speaker: voice.speaker, speed: request.speed, output: &output };
  let report = |value: f32| {
    let _ = app.emit(PROGRESS_EVENT, value);
  };
  match engine.speak(locate, &request, report).await {
    Ok(speech) => {
      takes.keep(kind, id);
      Ok(Take { id, duration_ms: speech.duration_ms, peaks: speech.peaks })
    }
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
