use std::fs::{self, File};
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use super::error::LibraryError;
use super::fingerprint::Fingerprint;
use super::names;

const INDEX: &str = "library.json";
const BACKUP: &str = "library.bak";
const TEMPORARY: &str = "library.part";
const DAMAGED: &str = "library.damaged";
const LOCK: &str = "library.lock";
/// The folder chosen for new recordings; absent means the default.
const SAVE_FOLDER: &str = "save-folder.json";
const SAVE_FOLDER_TEMPORARY: &str = "save-folder.part";
const PROBE: &str = ".voxmelody-probe";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Kind {
  Voice,
  Music,
  /// Music with singing.
  Song,
  Mix,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Entry {
  id: u64,
  kind: Kind,
  title: String,
  path: PathBuf,
  fingerprint: Fingerprint,
  duration_ms: u64,
  peaks: Vec<f32>,
  created_ms: u64,
  #[serde(default)]
  cloned: bool,
}

#[derive(Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Index {
  next_id: u64,
  entries: Vec<Entry>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Recording {
  pub id: u64,
  pub kind: Kind,
  pub title: String,
  pub duration_ms: u64,
  pub peaks: Vec<f32>,
  pub created_ms: u64,
  /// True when it contains a cloned voice.
  pub cloned: bool,
  /// False when the file was moved or deleted outside the app.
  pub available: bool,
}

pub struct NewRecording<'a> {
  pub kind: Kind,
  pub title: &'a str,
  pub source: &'a Path,
  pub duration_ms: u64,
  pub peaks: Vec<f32>,
  pub cloned: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveFolder {
  pub path: PathBuf,
  pub is_default: bool,
  /// False when the chosen folder cannot be reached, for example an unplugged drive.
  pub available: bool,
}

#[derive(Serialize, Deserialize)]
struct SavedFolder {
  path: PathBuf,
}

/// The index is read and written under a lock, so two open windows agree.
#[derive(Debug, Clone)]
pub struct Library {
  data: PathBuf,
  /// Where recordings go unless another folder was chosen.
  audio: PathBuf,
  fallback: PathBuf,
}

impl Library {
  /// Audio goes to `audio`, or beside the index when that folder is unusable.
  pub fn new(data: PathBuf, audio: PathBuf) -> std::io::Result<Self> {
    fs::create_dir_all(&data)?;
    let fallback = data.join("recordings");
    Ok(Self { data, audio, fallback })
  }

  /// Read on every use, so every window and the assistant agree.
  pub fn save_folder(&self) -> SaveFolder {
    match self.chosen_folder() {
      Some(path) => SaveFolder { available: path.is_dir(), path, is_default: false },
      None => SaveFolder { path: self.audio.clone(), is_default: true, available: true },
    }
  }

  /// Saves new recordings in `folder`, or in the default folder for `None`.
  /// Existing recordings stay where they are.
  pub fn set_save_folder(&self, folder: Option<&Path>) -> Result<(), LibraryError> {
    let _lock = self.lock()?;
    let Some(folder) = folder.filter(|folder| *folder != self.audio) else {
      return match fs::remove_file(self.data.join(SAVE_FOLDER)) {
        Err(error) if error.kind() != ErrorKind::NotFound => Err(error.into()),
        _ => Ok(()),
      };
    };
    fs::create_dir_all(folder)?;
    let probe = folder.join(PROBE);
    fs::write(&probe, b"")?;
    let _ = fs::remove_file(probe);
    let data = serde_json::to_vec(&SavedFolder { path: folder.to_path_buf() }).map_err(|_| LibraryError::Storage)?;
    let temporary = self.data.join(SAVE_FOLDER_TEMPORARY);
    fs::write(&temporary, data)?;
    Ok(fs::rename(temporary, self.data.join(SAVE_FOLDER))?)
  }

  fn chosen_folder(&self) -> Option<PathBuf> {
    let data = fs::read(self.data.join(SAVE_FOLDER)).ok()?;
    serde_json::from_slice::<SavedFolder>(&data).ok().map(|saved| saved.path)
  }

  /// Newest first.
  pub fn list(&self) -> Result<Vec<Recording>, LibraryError> {
    let _lock = self.lock()?;
    Ok(self.load()?.entries.iter().rev().map(|entry| self.recording(entry)).collect())
  }

  /// Moves a finished take into the library.
  pub fn add(&self, new: NewRecording) -> Result<Recording, LibraryError> {
    let title = names::title(new.title).unwrap_or_else(|| "VoxMelody".into());
    let _lock = self.lock()?;
    let mut index = self.load()?;
    let path = self.store(new.source, &title)?;
    let entry = Entry {
      id: index.next_id.max(1),
      kind: new.kind,
      title,
      fingerprint: Fingerprint::of(&path)?,
      path,
      duration_ms: new.duration_ms,
      peaks: new.peaks,
      created_ms: SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |age| age.as_millis() as u64),
      cloned: new.cloned,
    };
    index.next_id = entry.id + 1;
    index.entries.push(entry.clone());
    if let Err(error) = self.save(&index) {
      let _ = fs::remove_file(&entry.path);
      return Err(error);
    }
    let _ = fs::remove_file(new.source);
    Ok(self.recording(&entry))
  }

  /// The audio file and title of a recording.
  pub fn open(&self, id: u64) -> Result<(PathBuf, String), LibraryError> {
    self.open_where(id, |_| true)
  }

  /// The same, for a recording that must be of one kind.
  pub fn open_kind(&self, id: u64, kind: Kind) -> Result<(PathBuf, String), LibraryError> {
    self.open_where(id, |entry| entry.kind == kind)
  }

  pub fn is_cloned(&self, id: u64) -> Result<bool, LibraryError> {
    let _lock = self.lock()?;
    Ok(find(&self.load()?, id)?.cloned)
  }

  fn open_where(&self, id: u64, accepts: impl Fn(&Entry) -> bool) -> Result<(PathBuf, String), LibraryError> {
    let _lock = self.lock()?;
    let index = self.load()?;
    let entry = find(&index, id)?;
    let path = self.locate(entry).filter(|_| accepts(entry)).ok_or(LibraryError::Missing)?;
    Ok((path, entry.title.clone()))
  }

  /// Changes the title only, so files keep their names.
  pub fn rename(&self, id: u64, title: &str) -> Result<(), LibraryError> {
    let title = names::title(title).ok_or(LibraryError::Storage)?;
    let _lock = self.lock()?;
    let mut index = self.load()?;
    find_mut(&mut index, id)?.title = title;
    self.save(&index)
  }

  /// Deletes the recording and its file. A missing file is not an error.
  pub fn delete(&self, id: u64) -> Result<(), LibraryError> {
    let _lock = self.lock()?;
    let mut index = self.load()?;
    if let Some(path) = self.locate(find(&index, id)?) {
      fs::remove_file(path)?;
    }
    index.entries.retain(|entry| entry.id != id);
    self.save(&index)
  }

  /// Points a recording at its moved file, along with any others moved beside it.
  pub fn relink(&self, id: u64, chosen: &Path) -> Result<(), LibraryError> {
    let _lock = self.lock()?;
    let mut index = self.load()?;
    let entry = find_mut(&mut index, id)?;
    if !entry.fingerprint.matches(chosen) {
      return Err(LibraryError::Mismatch);
    }
    entry.path = chosen.to_path_buf();
    let folder = chosen.parent().unwrap_or(chosen);
    for entry in &mut index.entries {
      let moved = entry.path.file_name().map(|name| folder.join(name));
      if let Some(moved) = moved.filter(|moved| !self.is_available(entry) && entry.fingerprint.matches(moved)) {
        entry.path = moved;
      }
    }
    self.save(&index)
  }

  fn recording(&self, entry: &Entry) -> Recording {
    Recording {
      id: entry.id,
      kind: entry.kind,
      title: entry.title.clone(),
      duration_ms: entry.duration_ms,
      peaks: entry.peaks.clone(),
      created_ms: entry.created_ms,
      cloned: entry.cloned,
      available: self.is_available(entry),
    }
  }

  fn is_available(&self, entry: &Entry) -> bool {
    self.locate(entry).is_some()
  }

  /// The remembered path, or the same file back in a library folder.
  fn locate(&self, entry: &Entry) -> Option<PathBuf> {
    let name = entry.path.file_name()?;
    let chosen = self.chosen_folder().map(|folder| folder.join(name));
    [Some(entry.path.clone()), chosen, Some(self.audio.join(name)), Some(self.fallback.join(name))]
      .into_iter()
      .flatten()
      .find(|path| entry.fingerprint.matches(path))
  }

  fn store(&self, source: &Path, title: &str) -> std::io::Result<PathBuf> {
    store_in(&self.save_folder().path, source, title).or_else(|error| {
      log::warn!("library folder unavailable: {error}");
      store_in(&self.fallback, source, title)
    })
  }

  fn lock(&self) -> Result<File, LibraryError> {
    let file = File::options().create(true).truncate(false).write(true).open(self.data.join(LOCK))?;
    file.lock()?;
    Ok(file)
  }

  fn load(&self) -> Result<Index, LibraryError> {
    let (index, backup) = (self.data.join(INDEX), self.data.join(BACKUP));
    let current = read(&index)?;
    if let Some(Ok(current)) = current {
      return Ok(current);
    }
    if current.is_some() {
      log::warn!("library index is damaged; using the backup");
      fs::rename(&index, self.data.join(DAMAGED))?;
    }
    match read(&backup)? {
      Some(Ok(backup)) => Ok(backup),
      _ => Ok(Index::default()),
    }
  }

  fn save(&self, index: &Index) -> Result<(), LibraryError> {
    let data = serde_json::to_vec(index).map_err(|_| LibraryError::Storage)?;
    let (path, temporary) = (self.data.join(INDEX), self.data.join(TEMPORARY));
    fs::write(&temporary, data)?;
    File::options().write(true).open(&temporary)?.sync_all()?;
    if path.exists() {
      fs::rename(&path, self.data.join(BACKUP))?;
    }
    Ok(fs::rename(temporary, path)?)
  }
}

/// `None` when there is no file, and an inner error when its contents are damaged.
fn read(path: &Path) -> Result<Option<serde_json::Result<Index>>, LibraryError> {
  match fs::read(path) {
    Ok(data) => Ok(Some(serde_json::from_slice(&data))),
    Err(error) if error.kind() == ErrorKind::NotFound => Ok(None),
    Err(error) => Err(error.into()),
  }
}

fn find(index: &Index, id: u64) -> Result<&Entry, LibraryError> {
  index.entries.iter().find(|entry| entry.id == id).ok_or(LibraryError::Missing)
}

fn find_mut(index: &mut Index, id: u64) -> Result<&mut Entry, LibraryError> {
  index.entries.iter_mut().find(|entry| entry.id == id).ok_or(LibraryError::Missing)
}

fn store_in(dir: &Path, source: &Path, title: &str) -> std::io::Result<PathBuf> {
  fs::create_dir_all(dir)?;
  let path = names::reserve(dir, title)?;
  match fs::copy(source, &path) {
    Ok(_) => Ok(path),
    Err(error) => {
      let _ = fs::remove_file(&path);
      Err(error)
    }
  }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
