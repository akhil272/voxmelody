use std::fs::{self, File};
use std::io::ErrorKind;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use super::error::CloneError;
use super::sample::RATE;
use crate::audio::wav;
use crate::models::manifest::PackInfo;
use crate::voice::voices::Voice;

const INDEX: &str = "voices.json";
const TEMPORARY: &str = "voices.part";
const DAMAGED: &str = "voices.damaged";
const LOCK: &str = "voices.lock";
const NAME_LENGTH: usize = 40;
const PEAKS: usize = 160;

/// Whose voice it is, as confirmed when the voice was created.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Consent {
  Own,
  Permission,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClonedVoice {
  pub id: String,
  pub name: String,
  pub consent: Consent,
  /// When consent was confirmed.
  pub created_ms: u64,
  pub duration_ms: u64,
  pub peaks: Vec<f32>,
}

impl ClonedVoice {
  /// How the Voice Studio lists it.
  pub fn voice(&self, model: &PackInfo) -> Voice {
    Voice {
      id: self.id.clone(),
      model: model.id.clone(),
      name: self.name.clone(),
      language: model.languages.first().cloned().unwrap_or_default(),
      speaker: 0,
    }
  }
}

fn clean(name: &str) -> Result<String, CloneError> {
  let line = name.split_whitespace().collect::<Vec<_>>().join(" ");
  let name: String = line.chars().take(NAME_LENGTH).collect();
  if name.is_empty() { Err(CloneError::Name) } else { Ok(name) }
}

/// Samples and their index stay in the app data folder.
#[derive(Debug, Clone)]
pub struct Clones {
  dir: PathBuf,
}

impl Clones {
  pub fn new(dir: PathBuf) -> std::io::Result<Self> {
    fs::create_dir_all(&dir)?;
    Ok(Self { dir })
  }

  pub fn path(&self, voice: &ClonedVoice) -> PathBuf {
    self.dir.join(format!("{}.wav", voice.id))
  }

  /// Oldest first.
  pub fn list(&self) -> Result<Vec<ClonedVoice>, CloneError> {
    let _lock = self.lock()?;
    self.load()
  }

  pub fn find(&self, id: &str) -> Result<ClonedVoice, CloneError> {
    let voice = self.list()?.into_iter().find(|voice| voice.id == id).ok_or(CloneError::Missing)?;
    if self.path(&voice).exists() { Ok(voice) } else { Err(CloneError::Missing) }
  }

  pub fn add(&self, name: &str, consent: Consent, samples: &[f32]) -> Result<ClonedVoice, CloneError> {
    let name = clean(name)?;
    let _lock = self.lock()?;
    let mut voices = self.load()?;
    let created_ms = SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |age| age.as_millis() as u64);
    let taken = |number: &u64| voices.iter().any(|voice| voice.id == number.to_string());
    let id = (created_ms..).find(|number| !taken(number)).ok_or(CloneError::Storage)?.to_string();
    let duration_ms = samples.len() as u64 * 1000 / u64::from(RATE);
    let voice = ClonedVoice { id, name, consent, created_ms, duration_ms, peaks: wav::peaks(samples, PEAKS) };
    voices.push(voice.clone());
    let saved = wav::write_mono(&self.path(&voice), samples, RATE).map_err(CloneError::from).and_then(|()| self.save(&voices));
    if saved.is_err() {
      let _ = fs::remove_file(self.path(&voice));
    }
    saved.map(|()| voice)
  }

  pub fn rename(&self, id: &str, name: &str) -> Result<(), CloneError> {
    let name = clean(name)?;
    let _lock = self.lock()?;
    let mut voices = self.load()?;
    voices.iter_mut().find(|voice| voice.id == id).ok_or(CloneError::Missing)?.name = name;
    self.save(&voices)
  }

  /// Deletes the voice and its sample. A missing sample is not an error.
  pub fn delete(&self, id: &str) -> Result<(), CloneError> {
    let _lock = self.lock()?;
    let mut voices = self.load()?;
    let index = voices.iter().position(|voice| voice.id == id).ok_or(CloneError::Missing)?;
    let voice = voices.remove(index);
    match fs::remove_file(self.path(&voice)) {
      Err(error) if error.kind() != ErrorKind::NotFound => return Err(error.into()),
      _ => {}
    }
    self.save(&voices)
  }

  fn lock(&self) -> Result<File, CloneError> {
    let file = File::options().create(true).truncate(false).write(true).open(self.dir.join(LOCK))?;
    file.lock()?;
    Ok(file)
  }

  fn load(&self) -> Result<Vec<ClonedVoice>, CloneError> {
    let path = self.dir.join(INDEX);
    let data = match fs::read(&path) {
      Ok(data) => data,
      Err(error) if error.kind() == ErrorKind::NotFound => return Ok(Vec::new()),
      Err(error) => return Err(error.into()),
    };
    let voices: Result<Vec<ClonedVoice>, _> = serde_json::from_slice(&data);
    // Ids become file names, so anything else is damage.
    let is_safe = |voice: &ClonedVoice| !voice.id.is_empty() && voice.id.bytes().all(|byte| byte.is_ascii_digit());
    if let Some(voices) = voices.ok().filter(|voices| voices.iter().all(is_safe)) {
      return Ok(voices);
    }
    log::warn!("cloned voices index is damaged");
    fs::rename(&path, self.dir.join(DAMAGED))?;
    Ok(Vec::new())
  }

  fn save(&self, voices: &[ClonedVoice]) -> Result<(), CloneError> {
    let data = serde_json::to_vec(voices).map_err(|_| CloneError::Storage)?;
    let temporary = self.dir.join(TEMPORARY);
    fs::write(&temporary, data)?;
    File::options().write(true).open(&temporary)?.sync_all()?;
    Ok(fs::rename(temporary, self.dir.join(INDEX))?)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn clones(dir: &tempfile::TempDir) -> Clones {
    Clones::new(dir.path().join("vöices")).unwrap()
  }

  #[test]
  fn survives_restart_rename_and_delete() {
    let dir = tempfile::tempdir().unwrap();
    let first = clones(&dir).add("  Ana\nMaría ", Consent::Own, &[0.5; 24_000]).unwrap();
    let second = clones(&dir).add("Sam", Consent::Permission, &[0.5; 24_000]).unwrap();
    assert_ne!(first.id, second.id);
    let clones = clones(&dir);
    assert_eq!(clones.list().unwrap(), [first.clone(), second.clone()]);
    assert_eq!((first.name.as_str(), first.duration_ms), ("Ana María", 1000));
    clones.rename(&first.id, "Ana").unwrap();
    assert_eq!(clones.find(&first.id).unwrap().name, "Ana");
    assert_eq!(clones.rename(&first.id, " "), Err(CloneError::Name));
    let path = clones.path(&first);
    clones.delete(&first.id).unwrap();
    assert!(!path.exists());
    assert_eq!(clones.list().unwrap(), [second]);
    assert_eq!(clones.find(&first.id), Err(CloneError::Missing));
  }

  #[test]
  fn missing_samples_and_damage() {
    let dir = tempfile::tempdir().unwrap();
    let clones = clones(&dir);
    let voice = clones.add("Sam", Consent::Own, &[0.5; 10]).unwrap();
    fs::remove_file(clones.path(&voice)).unwrap();
    assert_eq!(clones.find(&voice.id), Err(CloneError::Missing));
    clones.delete(&voice.id).unwrap();
    let unsafe_id = serde_json::to_string(&[ClonedVoice { id: "../x".into(), ..voice }]).unwrap();
    for damage in ["{broken", unsafe_id.as_str()] {
      fs::write(clones.dir.join(INDEX), damage).unwrap();
      assert!(clones.list().unwrap().is_empty());
      assert!(clones.add("Next", Consent::Own, &[0.5; 10]).is_ok());
    }
  }
}
