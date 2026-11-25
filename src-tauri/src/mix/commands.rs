use std::path::Path;

use serde::Deserialize;
use tauri::{AppHandle, State};

use super::error::MixError;
use super::render::{self, Settings};
use crate::audio::{CHANNELS, wav};
use crate::library::commands as library;
use crate::library::store::{Kind, Library};
use crate::takes::{Take, Takes};

const PEAKS: usize = 160;
const VOLUME: std::ops::RangeInclusive<f32> = 0.0..=1.0;
const INTRO: std::ops::RangeInclusive<f32> = 0.0..=10.0;
const OUTRO: std::ops::RangeInclusive<f32> = 1.0..=10.0;

#[derive(Deserialize)]
pub struct MixRequest {
  voice: u64,
  music: u64,
  #[serde(flatten)]
  settings: Settings,
}

impl MixRequest {
  fn is_valid(&self) -> bool {
    let Settings { volume, intro, outro } = self.settings;
    VOLUME.contains(&volume) && INTRO.contains(&intro) && OUTRO.contains(&outro)
  }
}

struct Mixed {
  title: String,
  cloned: bool,
  duration_ms: u64,
  peaks: Vec<f32>,
}

fn create(library: &Library, request: &MixRequest, output: &Path) -> Result<Mixed, MixError> {
  let (voice, title) = library.open_kind(request.voice, Kind::Voice)?;
  let (music, _) = library.open_kind(request.music, Kind::Music)?;
  let samples = render::render(wav::read(&voice)?, wav::read(&music)?, &request.settings).ok_or(MixError::Audio)?;
  wav::write(output, &samples, render::RATE)?;
  let frames = (samples.len() / CHANNELS) as u64;
  let cloned = library.is_cloned(request.voice)?;
  Ok(Mixed { title, cloned, duration_ms: frames * 1000 / u64::from(render::RATE), peaks: wav::peaks(&samples, PEAKS) })
}

#[tauri::command]
pub async fn create_mix(
  app: AppHandle,
  library: State<'_, Library>,
  takes: State<'_, Takes>,
  request: MixRequest,
) -> Result<Take, MixError> {
  if !request.is_valid() {
    return Err(MixError::Request);
  }
  let (id, output) = takes.reserve();
  let (files, path) = (library.inner().clone(), output.clone());
  let mixed = tauri::async_runtime::spawn_blocking(move || create(&files, &request, &path)).await;
  let kept = match mixed.unwrap_or(Err(MixError::Output)) {
    Ok(mix) => {
      let finished = library::Finished {
        kind: Kind::Mix,
        title: &mix.title,
        source: output,
        duration_ms: mix.duration_ms,
        peaks: mix.peaks,
        cloned: mix.cloned,
      };
      library::keep(&app, finished).await.map_err(MixError::from)
    }
    Err(error) => Err(error),
  };
  kept.inspect_err(|_| takes.discard(id))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn checks_every_field() {
    let valid = |settings: &str| {
      let json = format!(r#"{{"voice":1,"music":2,{settings}}}"#);
      serde_json::from_str::<MixRequest>(&json).is_ok_and(|request| request.is_valid())
    };
    assert!(valid(r#""volume":0.5,"intro":0,"outro":3"#));
    assert!(!valid(r#""volume":1.1,"intro":0,"outro":3"#));
    assert!(!valid(r#""volume":1,"intro":11,"outro":3"#));
    assert!(!valid(r#""volume":1,"intro":0,"outro":0"#));
    assert!(!valid(r#""volume":1,"intro":0"#));
  }

  /// Mixes real recordings when the paths are set, for listening and transcription.
  #[test]
  fn mixes_real_recordings() {
    let path = |name| std::env::var_os(name);
    let (Some(voice), Some(music), Some(output)) = (
      path("VOXMELODY_TEST_MIX_VOICE"),
      path("VOXMELODY_TEST_MIX_MUSIC"),
      path("VOXMELODY_TEST_MIX_OUTPUT"),
    ) else {
      return;
    };
    let volume = std::env::var("VOXMELODY_TEST_MIX_VOLUME").map_or(0.5, |text| text.parse().unwrap());
    let settings = Settings { volume, intro: 2.0, outro: 3.0 };
    let (voice, music) = (wav::read(voice.as_ref()).unwrap(), wav::read(music.as_ref()).unwrap());
    wav::write(output.as_ref(), &render::render(voice, music, &settings).unwrap(), render::RATE).unwrap();
  }
}
