use std::path::PathBuf;

use tauri::ipc::Response;
use tauri::{AppHandle, Emitter, Manager, State};
use tauri_plugin_dialog::{DialogExt, FileDialogBuilder, FilePath};

use super::error::LibraryError;
use super::store::{Kind, Library, NewRecording, Recording, SaveFolder};
use crate::audio::wav;
use crate::takes::Take;

const CHANGED_EVENT: &str = "library://changed";
/// Stays in the file, so anyone can tell how it was made.
const CLONED_COMMENT: &str = "AI-generated speech in a cloned voice, made with VoxMelody";
const FOLDER_NAME: &str = "VoxMelody";

/// Recordings are ordinary files in the Music folder, so they outlive the app.
fn audio_dir(app: &AppHandle) -> tauri::Result<PathBuf> {
  #[cfg(debug_assertions)]
  if std::env::var_os("VOXMELODY_DATA_DIR").is_some() {
    return Ok(crate::models::commands::data_dir(app)?.join("audio"));
  }
  let folder = app.path().audio_dir().or_else(|_| app.path().app_local_data_dir())?;
  Ok(folder.join(FOLDER_NAME))
}

pub fn setup(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
  app.manage(Library::new(crate::models::commands::data_dir(app)?, audio_dir(app)?)?);
  Ok(())
}

/// Runs file work off the async threads.
async fn run<T: Send + 'static>(
  library: &Library,
  work: impl FnOnce(&Library) -> Result<T, LibraryError> + Send + 'static,
) -> Result<T, LibraryError> {
  let library = library.clone();
  tauri::async_runtime::spawn_blocking(move || work(&library)).await.map_err(|_| LibraryError::Storage)?
}

/// A take that is ready for the library.
pub struct Finished<'a> {
  pub kind: Kind,
  pub title: &'a str,
  pub source: PathBuf,
  pub duration_ms: u64,
  pub peaks: Vec<f32>,
  /// True when it contains a cloned voice.
  pub cloned: bool,
}

/// Keeps a finished take for good.
pub async fn keep(app: &AppHandle, finished: Finished<'_>) -> Result<Take, LibraryError> {
  let Finished { kind, source, duration_ms, peaks, cloned, .. } = finished;
  let title = finished.title.to_string();
  let recording = run(&app.state::<Library>(), move |library| {
    if cloned {
      wav::add_comment(&source, CLONED_COMMENT)?;
    }
    library.add(NewRecording { kind, title: &title, source: &source, duration_ms, peaks, cloned })
  })
  .await?;
  let _ = app.emit(CHANGED_EVENT, ());
  Ok(Take { id: recording.id, duration_ms: recording.duration_ms, peaks: recording.peaks })
}

/// Waits for a system dialog. `None` when the user closes it.
async fn choose(
  app: &AppHandle,
  open: impl FnOnce(FileDialogBuilder<tauri::Wry>, Box<dyn FnOnce(Option<FilePath>) + Send>),
) -> Result<Option<PathBuf>, LibraryError> {
  let (sender, receiver) = tokio::sync::oneshot::channel();
  let done = Box::new(move |file| {
    let _ = sender.send(file);
  });
  open(app.dialog().file().add_filter("WAV", &["wav"]), done);
  let Ok(Some(file)) = receiver.await else { return Ok(None) };
  file.into_path().map(Some).map_err(|_| LibraryError::Storage)
}

#[tauri::command]
pub async fn list_recordings(library: State<'_, Library>) -> Result<Vec<Recording>, LibraryError> {
  run(&library, |library| library.list()).await
}

#[tauri::command]
pub async fn read_recording(library: State<'_, Library>, id: u64) -> Result<Response, LibraryError> {
  run(&library, move |library| Ok(Response::new(std::fs::read(library.open(id)?.0)?))).await
}

#[tauri::command]
pub async fn rename_recording(library: State<'_, Library>, id: u64, title: String) -> Result<(), LibraryError> {
  run(&library, move |library| library.rename(id, &title)).await
}

#[tauri::command]
pub async fn delete_recording(library: State<'_, Library>, id: u64) -> Result<(), LibraryError> {
  run(&library, move |library| library.delete(id)).await
}

/// Returns false when the user closes the save dialog.
#[tauri::command]
pub async fn export_recording(
  app: AppHandle,
  library: State<'_, Library>,
  id: u64,
  name: String,
) -> Result<bool, LibraryError> {
  let (source, _) = run(&library, move |library| library.open(id)).await?;
  let Some(mut target) = choose(&app, |dialog, done| dialog.set_file_name(name).save_file(done)).await? else {
    return Ok(false);
  };
  if target.extension().is_none() {
    target.set_extension("wav");
  }
  // Copying a file onto itself would empty it.
  if std::fs::canonicalize(&source).ok() != std::fs::canonicalize(&target).ok() {
    tokio::fs::copy(source, target).await?;
  }
  Ok(true)
}

/// The path comes from the system file picker, never from the page.
#[tauri::command]
pub async fn relink_recording(app: AppHandle, library: State<'_, Library>, id: u64) -> Result<bool, LibraryError> {
  let Some(path) = choose(&app, |dialog, done| dialog.pick_file(done)).await? else { return Ok(false) };
  run(&library, move |library| library.relink(id, &path)).await?;
  Ok(true)
}

#[tauri::command]
pub async fn save_folder(library: State<'_, Library>) -> Result<SaveFolder, LibraryError> {
  run(&library, |library| Ok(library.save_folder())).await
}

/// Returns false when the user closes the folder picker. The path comes from the picker, never from the page.
#[tauri::command]
pub async fn change_save_folder(app: AppHandle, library: State<'_, Library>) -> Result<bool, LibraryError> {
  let (sender, receiver) = tokio::sync::oneshot::channel();
  app.dialog().file().pick_folder(move |folder| {
    let _ = sender.send(folder);
  });
  let Ok(Some(folder)) = receiver.await else { return Ok(false) };
  let folder = folder.into_path().map_err(|_| LibraryError::Storage)?;
  run(&library, move |library| library.set_save_folder(Some(&folder))).await?;
  Ok(true)
}

#[tauri::command]
pub async fn restore_save_folder(library: State<'_, Library>) -> Result<(), LibraryError> {
  run(&library, |library| library.set_save_folder(None)).await
}
