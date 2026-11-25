use std::collections::HashSet;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, Lines};
use tokio::process::{Child, ChildStdout};
use tokio::sync::{Mutex, MutexGuard, OnceCell, watch};

use super::error::MusicError;
use super::master;
use crate::audio::wav;
use crate::engine;
use crate::models::manifest::MusicModel;

/// A take that ends early is composed once more before it is extended.
const ATTEMPTS: u32 = 2;
/// Room to trim a silent start without shortening the track.
const SPARE_SECONDS: u32 = 2;
const WAVEFORM_BARS: usize = 160;

/// Fine control; zero or `None` keeps the model's own choice.
#[derive(Debug, Clone, Copy, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Options {
  pub seed: Option<u32>,
  pub steps: u32,
  pub guidance: f32,
  pub creativity: f32,
  pub cpu_only: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct Request<'a> {
  pub prompt: &'a str,
  pub seconds: u32,
  /// Zero lets the model choose.
  pub bpm: u32,
  /// Empty lets the model choose.
  pub key: &'a str,
  pub options: Options,
  pub model: &'a MusicModel,
  pub output: &'a Path,
}

#[derive(Serialize)]
#[serde(tag = "op", rename_all = "camelCase", rename_all_fields = "camelCase")]
enum Command<'a> {
  Devices,
  Compose {
    prompt: &'a str,
    seconds: u32,
    bpm: u32,
    key: &'a str,
    seed: Option<u32>,
    steps: u32,
    guidance: f32,
    creativity: f32,
    lm: &'a str,
    dit: &'a str,
    graphics_memory: u64,
    cpu_only: bool,
    models: &'a Path,
    output: &'a Path,
  },
}

#[derive(Debug, Deserialize)]
#[serde(tag = "event", rename_all = "camelCase")]
enum Event {
  Device { gpu: Option<String> },
  Progress { value: f32 },
  Done { rate: u32 },
  Error { code: MusicError },
}

#[derive(Debug)]
pub struct Track {
  pub duration_ms: u64,
  pub peaks: Vec<f32>,
}

struct Process {
  child: Child,
  events: Lines<BufReader<ChildStdout>>,
}

impl Process {
  /// The next event, or `None` once the engine has exited.
  async fn next(&mut self) -> Option<Event> {
    // Libraries may print their own text; only events are JSON.
    while let Ok(Some(line)) = self.events.next_line().await {
      if let Ok(event) = serde_json::from_str(&line) {
        return Some(event);
      }
    }
    None
  }

  /// Waits for exit, so no model file is still open afterwards.
  async fn stop(mut self) {
    let _ = self.child.kill().await;
  }
}

/// Only one track is made at a time.
#[derive(Default)]
struct Slot {
  /// Models whose GPU run fails where the CPU succeeds.
  cpu_only: HashSet<PathBuf>,
}

pub struct MusicEngine {
  program: PathBuf,
  slot: Mutex<Slot>,
  cancel: watch::Sender<u64>,
  graphics: OnceCell<Option<String>>,
}

/// While held, the engine is stopped and cannot restart.
pub struct Suspended<'a>(#[allow(dead_code)] MutexGuard<'a, Slot>);

impl MusicEngine {
  pub fn new(program: PathBuf) -> Self {
    Self { program, slot: Mutex::default(), cancel: watch::Sender::new(0), graphics: OnceCell::new() }
  }

  /// Stops current and queued requests.
  pub fn cancel(&self) {
    self.cancel.send_modify(|count| *count += 1);
  }

  /// Releases the model files so they can be moved or deleted.
  pub async fn suspend(&self) -> Suspended<'_> {
    self.cancel();
    Suspended(self.slot.lock().await)
  }

  /// Name of the graphics card that makes music, if there is one.
  pub async fn graphics_name(&self) -> Option<String> {
    self.graphics.get_or_init(|| self.find_graphics()).await.clone()
  }

  async fn find_graphics(&self) -> Option<String> {
    let mut process = self.start(&Command::Devices).await.ok()?;
    let event = process.next().await;
    process.stop().await;
    match event {
      Some(Event::Device { gpu }) => gpu,
      _ => None,
    }
  }

  /// `locate` runs once the engine is reserved and returns the music pack folder.
  pub async fn compose<F>(
    &self,
    locate: F,
    request: &Request<'_>,
    on_progress: impl Fn(f32),
  ) -> Result<Track, MusicError>
  where
    F: Future<Output = Result<PathBuf, MusicError>>,
  {
    let mut cancelled = self.cancel.subscribe();
    cancelled.mark_unchanged();
    let mut slot = tokio::select! {
      slot = self.slot.lock() => slot,
      _ = cancelled.changed() => return Err(MusicError::Cancelled),
    };
    let models = locate.await?;
    let raw = request.output.with_extension("raw");
    let mut job = Job { engine: self, models: &models, raw: &raw, request, on_progress, cancelled };
    let result = job.run(&mut slot).await;
    let _ = tokio::fs::remove_file(&raw).await;
    result
  }

  async fn start(&self, command: &Command<'_>) -> Result<Process, MusicError> {
    let mut child = engine::command(&self.program).spawn()?;
    let mut stdin = child.stdin.take().ok_or(MusicError::Engine)?;
    let stdout = child.stdout.take().ok_or(MusicError::Engine)?;
    let mut line = serde_json::to_vec(command).map_err(|_| MusicError::Request)?;
    line.push(b'\n');
    stdin.write_all(&line).await?;
    Ok(Process { child, events: BufReader::new(stdout).lines() })
  }
}

struct Job<'a, P> {
  engine: &'a MusicEngine,
  models: &'a Path,
  raw: &'a Path,
  request: &'a Request<'a>,
  on_progress: P,
  cancelled: watch::Receiver<u64>,
}

impl<P: Fn(f32)> Job<'_, P> {
  async fn run(&mut self, slot: &mut Slot) -> Result<Track, MusicError> {
    for attempt in 1..=ATTEMPTS {
      let rate = self.generate(slot, attempt).await?;
      if let Some(track) = self.finish(rate, attempt == ATTEMPTS).await? {
        return Ok(track);
      }
    }
    Err(MusicError::Engine)
  }

  /// Too little graphics memory or a driver fault falls back to the CPU.
  async fn generate(&mut self, slot: &mut Slot, attempt: u32) -> Result<u32, MusicError> {
    let mut used_gpu = false;
    let cpu_only = self.request.options.cpu_only || slot.cpu_only.contains(self.models);
    let result = self.process(cpu_only, attempt, &mut used_gpu).await;
    if !used_gpu || !matches!(result, Err(MusicError::Engine)) {
      return result;
    }
    log::warn!("music engine failed on the GPU; using the CPU");
    let result = self.process(true, attempt, &mut used_gpu).await;
    if result.is_ok() {
      slot.cpu_only.insert(self.models.to_path_buf());
    }
    result
  }

  /// Runs one engine process and returns the sample rate of the raw audio.
  async fn process(&mut self, cpu_only: bool, attempt: u32, used_gpu: &mut bool) -> Result<u32, MusicError> {
    let (options, model) = (self.request.options, self.request.model);
    let command = Command::Compose {
      prompt: self.request.prompt,
      seconds: self.request.seconds + SPARE_SECONDS,
      bpm: self.request.bpm,
      key: self.request.key,
      // Composing again needs other choices, but stays repeatable.
      seed: options.seed.map(|seed| seed.wrapping_add(attempt - 1)),
      steps: if options.steps == 0 { model.steps } else { options.steps },
      guidance: options.guidance,
      creativity: options.creativity,
      lm: &model.lm,
      dit: &model.dit,
      graphics_memory: model.graphics_memory,
      cpu_only,
      models: self.models,
      output: self.raw,
    };
    let mut process = self.engine.start(&command).await?;
    (self.on_progress)(0.0);
    let result = loop {
      let event = tokio::select! {
        event = process.next() => event,
        _ = self.cancelled.changed() => break Err(MusicError::Cancelled),
      };
      match event {
        Some(Event::Device { gpu }) => *used_gpu = gpu.is_some(),
        Some(Event::Progress { value }) => (self.on_progress)(value),
        Some(Event::Done { rate }) => break Ok(rate),
        Some(Event::Error { code }) => break Err(code),
        None => {
          log::warn!("music engine stopped unexpectedly");
          break Err(MusicError::Engine);
        }
      }
    };
    process.stop().await;
    result
  }

  /// Writes the finished track, or returns `None` when it should be composed again.
  async fn finish(&self, rate: u32, accept_short: bool) -> Result<Option<Track>, MusicError> {
    let (raw, output) = (self.raw.to_path_buf(), self.request.output.to_path_buf());
    let seconds = self.request.seconds;
    let work = move || {
      let samples = wav::read_raw(&raw).map_err(|_| MusicError::Output)?;
      if !accept_short && !master::is_long_enough(&samples, rate, seconds) {
        return Ok(None);
      }
      let track = master::finish(&samples, rate, seconds).ok_or(MusicError::Engine)?;
      wav::write(&output, &track, rate).map_err(|_| MusicError::Output)?;
      Ok(Some(Track { duration_ms: u64::from(seconds) * 1000, peaks: wav::peaks(&track, WAVEFORM_BARS) }))
    };
    tauri::async_runtime::spawn_blocking(work).await.map_err(|_| MusicError::Engine)?
  }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
