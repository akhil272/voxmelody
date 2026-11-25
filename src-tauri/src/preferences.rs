use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use tauri::AppHandle;

const FILE: &str = "preferences.json";
const TEMPORARY: &str = "preferences.part";

/// Stable codes the UI maps to user-facing copy.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum PreferenceError {
  Storage,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Preferences {
  advanced: bool,
  voice_model: String,
  music_model: String,
  favourites: Vec<String>,
}

fn dir(app: &AppHandle) -> Result<PathBuf, PreferenceError> {
  crate::models::commands::data_dir(app).map_err(|_| PreferenceError::Storage)
}

fn load(dir: &Path) -> Preferences {
  std::fs::read(dir.join(FILE)).ok().and_then(|data| serde_json::from_slice(&data).ok()).unwrap_or_default()
}

fn store(dir: &Path, preferences: &Preferences) -> std::io::Result<()> {
  std::fs::create_dir_all(dir)?;
  std::fs::write(dir.join(TEMPORARY), serde_json::to_vec(preferences)?)?;
  std::fs::rename(dir.join(TEMPORARY), dir.join(FILE))
}

#[tauri::command]
pub fn read_preferences(app: AppHandle) -> Result<Preferences, PreferenceError> {
  Ok(load(&dir(&app)?))
}

#[tauri::command]
pub fn save_preferences(app: AppHandle, preferences: Preferences) -> Result<(), PreferenceError> {
  store(&dir(&app)?, &preferences).map_err(|error| {
    log::warn!("preferences not saved: {error}");
    PreferenceError::Storage
  })
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn survives_restart_and_damage() {
    let dir = tempfile::tempdir().unwrap();
    assert_eq!(load(dir.path()), Preferences::default());
    let saved = Preferences { advanced: true, favourites: vec!["voice".into()], ..Default::default() };
    store(dir.path(), &saved).unwrap();
    store(dir.path(), &saved).unwrap();
    assert_eq!(load(dir.path()), saved);
    // Fields from another version are ignored or filled in.
    std::fs::write(dir.path().join(FILE), r#"{"advanced":true,"other":1}"#).unwrap();
    assert!(load(dir.path()).advanced);
    std::fs::write(dir.path().join(FILE), "{broken").unwrap();
    assert_eq!(load(dir.path()), Preferences::default());
  }
}
