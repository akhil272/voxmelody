//! Reads one JSON request per line on stdin and writes JSON events to stdout.
//! The working directory must be the installed voice pack: the speech library
//! cannot open non-ASCII paths on Windows, so every model path is relative.

mod engine;
mod text;
mod wav;

use std::io::{BufRead, Write};
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
  pub text: String,
  pub family: engine::Family,
  /// File name inside the working directory.
  pub model: String,
  pub language: String,
  pub speaker: i32,
  pub speed: f32,
  /// Length of pauses, where 1 is normal.
  pub pause: f32,
  /// Zero chooses automatically.
  pub threads: usize,
  pub steps: i32,
  pub output: PathBuf,
}

#[derive(Serialize)]
#[serde(tag = "event", rename_all = "camelCase")]
pub enum Event {
  Progress { value: f32 },
  #[serde(rename_all = "camelCase")]
  Done { duration_ms: u64, peaks: Vec<f32> },
  Error { code: ErrorCode },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ErrorCode {
  Model,
  NoSpeech,
  Output,
  Request,
}

pub fn emit(event: &Event) {
  if let Ok(line) = serde_json::to_string(event) {
    let mut stdout = std::io::stdout().lock();
    let _ = writeln!(stdout, "{line}");
    let _ = stdout.flush();
  }
}

fn main() {
  let mut engine = engine::Engine::default();
  // Ends on end-of-file, so the process never outlives the app.
  for line in std::io::stdin().lock().lines().map_while(Result::ok) {
    let result = serde_json::from_str::<Request>(&line)
      .map_err(|_| ErrorCode::Request)
      .and_then(|request| engine.speak(&request));
    emit(&result.unwrap_or_else(|code| Event::Error { code }));
  }
}
