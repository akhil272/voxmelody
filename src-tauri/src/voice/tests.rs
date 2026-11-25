//! Runs against a real engine and voice pack when both are available:
//! VOXMELODY_TEST_ENGINE and VOXMELODY_TEST_VOICE.

use super::*;

fn setup() -> Option<(VoiceEngine, PathBuf, tempfile::TempDir)> {
  let engine = std::env::var_os("VOXMELODY_TEST_ENGINE")?;
  let pack = std::env::var_os("VOXMELODY_TEST_VOICE")?;
  Some((VoiceEngine::new(engine.into()), pack.into(), tempfile::tempdir().ok()?))
}

fn request<'a>(text: &'a str, output: &'a Path) -> Request<'a> {
  Request {
    family: "kokoro",
    model: "model.onnx",
    text,
    language: "en-us",
    speaker: 3,
    speed: 1.0,
    pause: 1.0,
    threads: 0,
    steps: 0,
    output,
  }
}

#[tokio::test]
async fn missing_engine_is_an_error() {
  let dir = tempfile::tempdir().unwrap();
  let engine = VoiceEngine::new(dir.path().join("nope.exe"));
  let output = dir.path().join("a.wav");
  let pack = dir.path().to_path_buf();
  let result = engine.speak(async { Ok(pack) }, &request("hi", &output), |_| {}).await;
  assert_eq!(result.unwrap_err(), VoiceError::Engine);
}

#[tokio::test]
async fn speaks_recovers_and_suspends() {
  let Some((engine, pack, dir)) = setup() else { return };
  let output = dir.path().join("é.wav");
  let speech = engine.speak(async { Ok(pack.clone()) }, &request("Hello there.", &output), |_| {}).await.unwrap();
  assert!(speech.duration_ms > 400 && !speech.peaks.is_empty() && output.exists());

  let empty = engine.speak(async { Ok(pack.clone()) }, &request("...", &output), |_| {}).await;
  assert_eq!(empty.unwrap_err(), VoiceError::NoSpeech);

  // A crash fails one request; the next starts a fresh process.
  engine.slot.lock().await.process.as_mut().unwrap().child.kill().await.unwrap();
  let again = engine.speak(async { Ok(pack.clone()) }, &request("Still here.", &output), |_| {}).await;
  assert!(again.is_ok());

  let suspended = engine.suspend().await;
  assert!(suspended.0.process.is_none());
}

#[tokio::test]
async fn cancels_running_and_queued_requests() {
  let Some((engine, pack, dir)) = setup() else { return };
  let long = "The river moved slowly past the old town. ".repeat(60);
  let (first, second) = (dir.path().join("a.wav"), dir.path().join("b.wav"));
  let (first_request, second_request) = (request(&long, &first), request("Queued.", &second));
  let started = std::time::Instant::now();
  let (running, queued, ()) = tokio::join!(
    engine.speak(async { Ok(pack.clone()) }, &first_request, |_| {}),
    engine.speak(async { Ok(pack.clone()) }, &second_request, |_| {}),
    async {
      tokio::time::sleep(Duration::from_secs(4)).await;
      engine.cancel();
    }
  );
  assert_eq!(running.unwrap_err(), VoiceError::Cancelled);
  assert_eq!(queued.unwrap_err(), VoiceError::Cancelled);
  assert!(started.elapsed() < Duration::from_secs(6));
  assert!(!first.exists() && !second.exists());
  assert!(engine.speak(async { Ok(pack.clone()) }, &request("After.", &second), |_| {}).await.is_ok());
}
