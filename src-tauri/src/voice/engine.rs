use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use serde::{Deserialize, Serialize};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, Lines};
use tokio::process::{Child, ChildStdin, ChildStdout};
use tokio::sync::{Mutex, MutexGuard, watch};

use super::error::VoiceError;

/// Frees the model's memory after a quiet period.
const IDLE_TIMEOUT: Duration = Duration::from_secs(120);

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Request<'a> {
  pub family: &'a str,
  pub model: &'a str,
  pub text: &'a str,
  pub language: &'a str,
  pub speaker: i32,
  pub speed: f32,
  pub pause: f32,
  pub threads: u32,
  pub steps: u32,
  pub output: &'a Path,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "event", rename_all = "camelCase")]
enum Event {
  Progress { value: f32 },
  #[serde(rename_all = "camelCase")]
  Done { duration_ms: u64, peaks: Vec<f32> },
  Error { code: VoiceError },
}

#[derive(Debug)]
pub struct Speech {
  pub duration_ms: u64,
  pub peaks: Vec<f32>,
}

struct Process {
  child: Child,
  stdin: ChildStdin,
  stdout: Lines<BufReader<ChildStdout>>,
  dir: PathBuf,
}

#[derive(Default)]
struct Slot {
  process: Option<Process>,
  /// Counts uses so an old idle timer never stops a newer session.
  uses: u64,
}

pub struct VoiceEngine {
  program: PathBuf,
  slot: Arc<Mutex<Slot>>,
  cancel: watch::Sender<u64>,
}

/// While held, the engine is stopped and cannot restart.
pub struct Suspended<'a>(#[allow(dead_code)] MutexGuard<'a, Slot>);

impl VoiceEngine {
  pub fn new(program: PathBuf) -> Self {
    Self { program, slot: Arc::default(), cancel: watch::Sender::new(0) }
  }

  /// Stops current and queued requests.
  pub fn cancel(&self) {
    self.cancel.send_modify(|count| *count += 1);
  }

  /// Releases the model files so they can be moved or deleted.
  pub async fn suspend(&self) -> Suspended<'_> {
    self.cancel();
    let mut slot = self.slot.lock().await;
    stop(&mut slot).await;
    Suspended(slot)
  }

  /// `locate` runs once the engine is reserved and returns the voice pack folder.
  pub async fn speak<F>(
    &self,
    locate: F,
    request: &Request<'_>,
    on_progress: impl Fn(f32),
  ) -> Result<Speech, VoiceError>
  where
    F: Future<Output = Result<PathBuf, VoiceError>>,
  {
    let mut cancelled = self.cancel.subscribe();
    cancelled.mark_unchanged();
    let mut slot = tokio::select! {
      slot = self.slot.lock() => slot,
      _ = cancelled.changed() => return Err(VoiceError::Cancelled),
    };
    let dir = locate.await?;
    let result = tokio::select! {
      result = self.run(&mut slot, &dir, request, &on_progress) => result,
      _ = cancelled.changed() => Err(VoiceError::Cancelled),
    };
    slot.uses += 1;
    if matches!(result, Err(VoiceError::Cancelled | VoiceError::Engine)) {
      stop(&mut slot).await;
    } else {
      self.stop_when_idle(slot.uses);
    }
    result
  }

  async fn run(
    &self,
    slot: &mut Slot,
    dir: &Path,
    request: &Request<'_>,
    on_progress: &impl Fn(f32),
  ) -> Result<Speech, VoiceError> {
    let running = match &mut slot.process {
      Some(process) => process.dir == dir && process.child.try_wait()?.is_none(),
      None => false,
    };
    if !running {
      stop(slot).await;
      slot.process = Some(self.spawn(dir)?);
    }
    let process = slot.process.as_mut().ok_or(VoiceError::Engine)?;
    let mut line = serde_json::to_vec(request).map_err(|_| VoiceError::Request)?;
    line.push(b'\n');
    process.stdin.write_all(&line).await?;
    process.stdin.flush().await?;
    while let Some(line) = process.stdout.next_line().await? {
      // Libraries may print their own text; only events are JSON.
      match serde_json::from_str(&line) {
        Ok(Event::Progress { value }) => on_progress(value),
        Ok(Event::Done { duration_ms, peaks }) => return Ok(Speech { duration_ms, peaks }),
        Ok(Event::Error { code }) => return Err(code),
        Err(_) => {}
      }
    }
    log::warn!("voice engine stopped unexpectedly");
    Err(VoiceError::Engine)
  }

  fn spawn(&self, dir: &Path) -> Result<Process, VoiceError> {
    let mut child = crate::engine::command(&self.program).current_dir(dir).spawn()?;
    let stdin = child.stdin.take().ok_or(VoiceError::Engine)?;
    let stdout = child.stdout.take().ok_or(VoiceError::Engine)?;
    Ok(Process { child, stdin, stdout: BufReader::new(stdout).lines(), dir: dir.into() })
  }

  fn stop_when_idle(&self, uses: u64) {
    let slot = self.slot.clone();
    tauri::async_runtime::spawn(async move {
      tokio::time::sleep(IDLE_TIMEOUT).await;
      let mut slot = slot.lock().await;
      if slot.uses == uses {
        stop(&mut slot).await;
      }
    });
  }
}

async fn stop(slot: &mut Slot) {
  if let Some(mut process) = slot.process.take() {
    let _ = process.child.kill().await;
  }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
