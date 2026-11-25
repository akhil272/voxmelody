use std::sync::Mutex;

use serde::Serialize;
use tauri::ipc::{InvokeBody, Request, Response};
use tauri::{AppHandle, Manager, State};

use super::error::CloneError;
use super::sample::{self, RATE};
use super::store::{ClonedVoice, Clones, Consent};
use crate::audio::wav;

const PEAKS: usize = 160;
/// Five minutes of audio; the sample itself is far shorter.
const MAX_BYTES: usize = 5 * 60 * RATE as usize * size_of::<f32>();

/// A checked recording waiting for a name and consent.
#[derive(Default)]
pub struct PendingSample(Mutex<Option<Vec<f32>>>);

impl PendingSample {
  fn set(&self, samples: Option<Vec<f32>>) {
    *self.0.lock().unwrap_or_else(|poisoned| poisoned.into_inner()) = samples;
  }

  fn get(&self) -> Option<Vec<f32>> {
    self.0.lock().unwrap_or_else(|poisoned| poisoned.into_inner()).clone()
  }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Sample {
  duration_ms: u64,
  peaks: Vec<f32>,
}

pub fn setup(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
  app.manage(Clones::new(crate::models::commands::data_dir(app)?.join("voices"))?);
  app.manage(PendingSample::default());
  Ok(())
}

/// Takes mono float samples and keeps the prepared recording until it is saved.
#[tauri::command]
pub async fn check_sample(request: Request<'_>, pending: State<'_, PendingSample>) -> Result<Sample, CloneError> {
  pending.set(None);
  let InvokeBody::Raw(data) = request.body() else { return Err(CloneError::Audio) };
  let samples = sample::prepare(&sample::decode(&data[..data.len().min(MAX_BYTES)])?)?;
  let duration_ms = samples.len() as u64 * 1000 / u64::from(RATE);
  let peaks = wav::peaks(&samples, PEAKS);
  pending.set(Some(samples));
  Ok(Sample { duration_ms, peaks })
}

/// The prepared recording, so people hear what will be used.
#[tauri::command]
pub async fn read_sample(pending: State<'_, PendingSample>) -> Result<Response, CloneError> {
  let samples = pending.get().ok_or(CloneError::Audio)?;
  Ok(Response::new(wav::mono(&samples, RATE)?))
}

#[tauri::command]
pub fn discard_sample(pending: State<'_, PendingSample>) {
  pending.set(None);
}

/// Saves the checked recording. Consent is required.
#[tauri::command]
pub async fn create_clone(
  clones: State<'_, Clones>,
  pending: State<'_, PendingSample>,
  name: String,
  consent: Option<Consent>,
) -> Result<ClonedVoice, CloneError> {
  let consent = consent.ok_or(CloneError::Consent)?;
  let samples = pending.get().ok_or(CloneError::Audio)?;
  let voice = clones.add(&name, consent, &samples)?;
  pending.set(None);
  Ok(voice)
}

#[tauri::command]
pub async fn list_clones(clones: State<'_, Clones>) -> Result<Vec<ClonedVoice>, CloneError> {
  clones.list()
}

#[tauri::command]
pub async fn read_clone(clones: State<'_, Clones>, id: String) -> Result<Response, CloneError> {
  Ok(Response::new(tokio::fs::read(clones.path(&clones.find(&id)?)).await?))
}

#[tauri::command]
pub async fn rename_clone(clones: State<'_, Clones>, id: String, name: String) -> Result<(), CloneError> {
  clones.rename(&id, &name)
}

#[tauri::command]
pub async fn delete_clone(clones: State<'_, Clones>, id: String) -> Result<(), CloneError> {
  clones.delete(&id)
}
