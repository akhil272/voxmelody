use serde::Deserialize;
use tauri::{AppHandle, Emitter, Manager, State};

use super::engine::{MusicEngine, Request};
use super::error::MusicError;
use crate::library::commands as library;
use crate::library::store::Kind;
use crate::models::manager::ModelManager;
use crate::takes::{Take, Takes};

const PROGRESS_EVENT: &str = "music://progress";
const MAX_CHARACTERS: usize = 500;
const SECONDS: std::ops::RangeInclusive<u32> = 10..=240;
const BPM: std::ops::RangeInclusive<u32> = 30..=300;
const NOTES: [&str; 12] = ["C", "C#", "D", "Eb", "E", "F", "F#", "G", "Ab", "A", "Bb", "B"];
const MODES: [&str; 2] = ["major", "minor"];

#[derive(Deserialize)]
pub struct MusicRequest {
  prompt: String,
  seconds: u32,
  /// Zero lets the model choose.
  bpm: u32,
  /// Empty lets the model choose.
  key: String,
}

impl MusicRequest {
  fn is_valid(&self) -> bool {
    let prompt = self.prompt.trim();
    let key = self.key.split_once(' ').is_some_and(|(note, mode)| NOTES.contains(&note) && MODES.contains(&mode));
    !prompt.is_empty()
      && prompt.chars().count() <= MAX_CHARACTERS
      && SECONDS.contains(&self.seconds)
      && (self.bpm == 0 || BPM.contains(&self.bpm))
      && (self.key.is_empty() || key)
  }
}

pub fn setup(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
  app.manage(MusicEngine::new(crate::engine::path("voxmelody-music")?));
  Ok(())
}

#[tauri::command]
pub async fn compose(
  app: AppHandle,
  engine: State<'_, MusicEngine>,
  models: State<'_, ModelManager>,
  takes: State<'_, Takes>,
  request: MusicRequest,
) -> Result<Take, MusicError> {
  if !request.is_valid() {
    return Err(MusicError::Request);
  }
  let (id, output) = takes.reserve();
  let locate = async { models.installed_dir("music").await.ok_or(MusicError::NotInstalled) };
  let request = Request {
    prompt: request.prompt.trim(),
    seconds: request.seconds,
    bpm: request.bpm,
    key: &request.key,
    output: &output,
  };
  let report = |value: f32| {
    let _ = app.emit(PROGRESS_EVENT, value);
  };
  match engine.compose(locate, &request, report).await {
    Ok(track) => {
      let kept = library::keep(&app, Kind::Music, request.prompt, output.clone(), track.duration_ms, track.peaks).await;
      kept.map_err(|_| {
        takes.discard(id);
        MusicError::Output
      })
    }
    Err(error) => {
      takes.discard(id);
      Err(error)
    }
  }
}

#[tauri::command]
pub fn cancel_music(engine: State<'_, MusicEngine>) {
  engine.cancel();
}

#[tauri::command]
pub async fn graphics_name(engine: State<'_, MusicEngine>) -> Result<Option<String>, MusicError> {
  Ok(engine.graphics_name().await)
}

#[cfg(test)]
mod tests {
  use super::*;

  fn request(prompt: &str, seconds: u32, bpm: u32, key: &str) -> MusicRequest {
    MusicRequest { prompt: prompt.into(), seconds, bpm, key: key.into() }
  }

  #[test]
  fn checks_every_field() {
    assert!(request("Jazz", 30, 0, "").is_valid());
    assert!(request(&"é".repeat(500), 240, 120, "F# minor").is_valid());
    assert!(!request(" ", 30, 0, "").is_valid());
    assert!(!request(&"a".repeat(501), 30, 0, "").is_valid());
    assert!(!request("Jazz", 9, 0, "").is_valid());
    assert!(!request("Jazz", 30, 301, "").is_valid());
    assert!(!request("Jazz", 30, 0, "H major").is_valid());
    assert!(!request("Jazz", 30, 0, "C major extra").is_valid());
  }
}
