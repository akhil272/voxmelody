use std::path::PathBuf;
use std::sync::Arc;

use serde::Serialize;
use sysinfo::{MemoryRefreshKind, RefreshKind, System};
use tauri::{AppHandle, Emitter, Manager, State};
use tauri_plugin_dialog::DialogExt;

use super::error::ModelError;
use super::location::LocationStore;
use super::manager::{LocationStatus, ModelManager, PackStatus};
use super::manifest::{self, Pack, PackInfo, PackKind};
use crate::music::engine::MusicEngine;
use crate::voice::engine::VoiceEngine;

const STATUS_EVENT: &str = "models://status";
const LOCATION_EVENT: &str = "models://location";

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemInfo {
  memory_bytes: u64,
  free_disk_bytes: Option<u64>,
  cpu_threads: usize,
}

/// Debug builds accept an override so tests never touch real user data.
pub fn data_dir(app: &AppHandle) -> tauri::Result<PathBuf> {
  #[cfg(debug_assertions)]
  if let Some(dir) = std::env::var_os("VOXMELODY_DATA_DIR") {
    return Ok(dir.into());
  }
  app.path().app_local_data_dir()
}

pub fn setup(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
  let (packs, locations) = (app.clone(), app.clone());
  let data = data_dir(app)?;
  let store = LocationStore::new(data.join("settings.json"), data.join("models"));
  let manager = ModelManager::new(
    store,
    Arc::new(move |status: PackStatus| emit(&packs, STATUS_EVENT, status)),
    Arc::new(move |status: LocationStatus| emit(&locations, LOCATION_EVENT, status)),
  )
  .map_err(|error| format!("{error:?}"))?;
  app.manage(manager);
  Ok(())
}

fn emit(app: &AppHandle, event: &str, payload: impl Serialize + Clone) {
  if let Err(error) = app.emit(event, payload) {
    log::warn!("could not emit {event}: {error}");
  }
}

fn find(id: &str) -> Result<&'static Pack, ModelError> {
  manifest::find(id).ok_or(ModelError::UnknownPack)
}

#[tauri::command]
pub fn list_models() -> Vec<&'static PackInfo> {
  manifest::packs().iter().map(|pack| &pack.info).collect()
}

#[tauri::command]
pub async fn list_packs(manager: State<'_, ModelManager>) -> Result<Vec<PackStatus>, ModelError> {
  let mut statuses = Vec::new();
  for pack in manifest::packs() {
    statuses.push(manager.status(pack).await);
  }
  Ok(statuses)
}

#[tauri::command]
pub async fn download_pack(manager: State<'_, ModelManager>, id: String) -> Result<(), ModelError> {
  manager.start(find(&id)?).await
}

#[tauri::command]
pub async fn pause_pack(manager: State<'_, ModelManager>, id: String) -> Result<(), ModelError> {
  manager.pause(find(&id)?).await;
  Ok(())
}

#[tauri::command]
pub async fn remove_pack(
  manager: State<'_, ModelManager>,
  voice: State<'_, VoiceEngine>,
  music: State<'_, MusicEngine>,
  id: String,
) -> Result<(), ModelError> {
  let pack = find(&id)?;
  // Removing one model never interrupts the other studio.
  let _suspended = match pack.kind {
    PackKind::Music => (None, Some(music.suspend().await)),
    PackKind::Voice => (Some(voice.suspend().await), None),
  };
  manager.remove(pack).await
}

#[tauri::command]
pub async fn models_location(manager: State<'_, ModelManager>) -> Result<LocationStatus, ModelError> {
  Ok(manager.location())
}

/// Opens the system folder picker; the path never comes from the page.
#[tauri::command]
pub async fn change_models_location(
  app: AppHandle,
  manager: State<'_, ModelManager>,
  voice: State<'_, VoiceEngine>,
  music: State<'_, MusicEngine>,
) -> Result<(), ModelError> {
  let (sender, receiver) = tokio::sync::oneshot::channel();
  app.dialog().file().pick_folder(move |folder| {
    let _ = sender.send(folder);
  });
  let Ok(Some(folder)) = receiver.await else { return Ok(()) };
  let path = folder.into_path().map_err(|_| ModelError::Storage)?;
  let _suspended = (voice.suspend().await, music.suspend().await);
  manager.change_location(&path).await
}

#[tauri::command]
pub async fn restore_models_location(
  manager: State<'_, ModelManager>,
  voice: State<'_, VoiceEngine>,
  music: State<'_, MusicEngine>,
) -> Result<(), ModelError> {
  let _suspended = (voice.suspend().await, music.suspend().await);
  manager.restore_location().await
}

#[tauri::command]
pub async fn system_info(manager: State<'_, ModelManager>) -> Result<SystemInfo, ModelError> {
  let system = System::new_with_specifics(
    RefreshKind::nothing().with_memory(MemoryRefreshKind::nothing().with_ram()),
  );
  Ok(SystemInfo {
    memory_bytes: system.total_memory(),
    free_disk_bytes: manager.free_space().ok(),
    cpu_threads: std::thread::available_parallelism().map_or(1, usize::from),
  })
}
