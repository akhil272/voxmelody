use serde::Deserialize;
use tauri::{AppHandle, Emitter, Manager, State};

use super::engine::{MusicEngine, Options, Request, Song};
use super::error::MusicError;
use super::song::{self, SongRequest};
use crate::library::commands as library;
use crate::library::store::Kind;
use crate::models::manager::ModelManager;
use crate::models::manifest::{self, PackKind};
use crate::takes::{Take, Takes};

const PROGRESS_EVENT: &str = "music://progress";
const MAX_CHARACTERS: usize = 500;
const SECONDS: std::ops::RangeInclusive<u32> = 10..=240;
const BPM: std::ops::RangeInclusive<u32> = 30..=300;
const MAX_STEPS: u32 = 100;
const GUIDANCE: std::ops::RangeInclusive<f32> = 0.0..=15.0;
const CREATIVITY: std::ops::RangeInclusive<f32> = 0.0..=1.5;
const NOTES: [&str; 12] = ["C", "C#", "D", "Eb", "E", "F", "F#", "G", "Ab", "A", "Bb", "B"];
const MODES: [&str; 2] = ["major", "minor"];

#[derive(Deserialize)]
pub struct MusicRequest {
  pub model: String,
  pub prompt: String,
  /// Zero fits a song to its lyrics.
  pub seconds: u32,
  /// Zero lets the model choose.
  pub bpm: u32,
  /// Empty lets the model choose.
  pub key: String,
  /// What is sung; left out for an instrumental.
  pub song: Option<SongRequest>,
  pub options: Options,
}

impl MusicRequest {
  fn has_valid_length(&self) -> bool {
    match &self.song {
      None => SECONDS.contains(&self.seconds),
      Some(song) => song.is_valid() && (self.seconds == 0 || song::SECONDS.contains(&self.seconds)),
    }
  }

  fn is_valid(&self) -> bool {
    let prompt = self.prompt.trim();
    let key = self.key.split_once(' ').is_some_and(|(note, mode)| NOTES.contains(&note) && MODES.contains(&mode));
    !prompt.is_empty()
      && prompt.chars().count() <= MAX_CHARACTERS
      && self.has_valid_length()
      && (self.bpm == 0 || BPM.contains(&self.bpm))
      && (self.key.is_empty() || key)
      && self.options.steps <= MAX_STEPS
      && GUIDANCE.contains(&self.options.guidance)
      && CREATIVITY.contains(&self.options.creativity)
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
  let pack = manifest::find(&request.model).ok_or(MusicError::Request)?;
  let model = pack.music.as_ref().ok_or(MusicError::Request)?;
  let (id, output) = takes.reserve();
  let locate = async {
    models.installed(&pack.id, PackKind::Music).await.map(|(dir, _)| dir).ok_or(MusicError::NotInstalled)
  };
  let song = request.song.as_ref();
  let request = Request {
    prompt: request.prompt.trim(),
    seconds: song.filter(|_| request.seconds == 0).map_or(request.seconds, SongRequest::seconds),
    song: song.map(|song| Song { lyrics: song.lyrics.trim(), language: &song.language }),
    bpm: request.bpm,
    key: &request.key,
    options: request.options,
    model,
    output: &output,
  };
  let report = |value: f32| {
    let _ = app.emit(PROGRESS_EVENT, value);
  };
  match engine.compose(locate, &request, report).await {
    Ok(track) => {
      let finished = library::Finished {
        kind: if song.is_some() { Kind::Song } else { Kind::Music },
        title: request.prompt,
        source: output.clone(),
        duration_ms: track.duration_ms,
        peaks: track.peaks,
        cloned: false,
      };
      library::keep(&app, finished).await.map_err(|_| {
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
    MusicRequest { model: "music".into(), prompt: prompt.into(), seconds, bpm, key: key.into(), song: None, options: Options::default() }
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
    let sing = |seconds| {
      let song = Some(SongRequest { lyrics: "Hello there".into(), language: "en".into() });
      MusicRequest { song, ..request("Jazz", seconds, 0, "") }.is_valid()
    };
    assert!(sing(0) && sing(240) && !sing(10) && !sing(241));
    assert!(!request("Jazz", 0, 0, "").is_valid());
  }
}
