use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use tauri::ipc::Response;
use tauri::{AppHandle, Emitter, Manager, State};
use tauri_plugin_dialog::DialogExt;

use super::engine::{Request, VoiceEngine};
use super::error::VoiceError;
use super::takes::{TakeKind, Takes};
use super::voices::{self, VOICES, Voice};
use crate::models::manager::ModelManager;

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

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Take {
  id: u64,
  duration_ms: u64,
  peaks: Vec<f32>,
}

/// Engines sit beside the app executable.
fn engine_path() -> std::io::Result<PathBuf> {
  let program = std::env::current_exe()?;
  Ok(program.with_file_name(format!("voxmelody-voice{}", std::env::consts::EXE_SUFFIX)))
}

pub fn setup(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
  let cache = crate::models::commands::data_dir(app)?.join("cache").join("voice");
  app.manage(VoiceEngine::new(engine_path()?));
  app.manage(Takes::new(cache)?);
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
    (request.text.as_str(), TakeKind::Full)
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

#[tauri::command]
pub async fn read_take(takes: State<'_, Takes>, id: u64) -> Result<Response, VoiceError> {
  let path = takes.find(id).ok_or(VoiceError::Output)?;
  Ok(Response::new(tokio::fs::read(path).await.map_err(|_| VoiceError::Output)?))
}

/// Returns false when the user closes the save dialog.
#[tauri::command]
pub async fn export_take(
  app: AppHandle,
  takes: State<'_, Takes>,
  id: u64,
  name: String,
) -> Result<bool, VoiceError> {
  let source = takes.find(id).ok_or(VoiceError::Output)?;
  let (sender, receiver) = tokio::sync::oneshot::channel();
  app.dialog().file().set_file_name(name).add_filter("WAV", &["wav"]).save_file(move |file| {
    let _ = sender.send(file);
  });
  let Ok(Some(file)) = receiver.await else { return Ok(false) };
  let mut target = file.into_path().map_err(|_| VoiceError::Output)?;
  if target.extension().is_none() {
    target.set_extension("wav");
  }
  tokio::fs::copy(source, target).await.map_err(|_| VoiceError::Output)?;
  Ok(true)
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
