//! Runs against a real engine and music pack when both are available:
//! VOXMELODY_TEST_MUSIC_ENGINE and VOXMELODY_TEST_MUSIC.

use super::*;

fn setup() -> Option<(MusicEngine, PathBuf, tempfile::TempDir)> {
  let engine = std::env::var_os("VOXMELODY_TEST_MUSIC_ENGINE")?;
  let pack = std::env::var_os("VOXMELODY_TEST_MUSIC")?;
  Some((MusicEngine::new(engine.into()), pack.into(), tempfile::tempdir().ok()?))
}

fn request(output: &Path) -> Request<'_> {
  let model = crate::models::manifest::find("music").unwrap().music.as_ref().unwrap();
  Request { prompt: "Calm acoustic guitar", seconds: 10, bpm: 90, key: "G major", options: Options::default(), model, output }
}

#[tokio::test]
async fn missing_engine_is_an_error() {
  let dir = tempfile::tempdir().unwrap();
  let engine = MusicEngine::new(dir.path().join("nope.exe"));
  let output = dir.path().join("a.wav");
  let pack = dir.path().to_path_buf();
  let result = engine.compose(async { Ok(pack) }, &request(&output), |_| {}).await;
  assert_eq!(result.unwrap_err(), MusicError::Engine);
  assert_eq!(engine.graphics_name().await, None);
}

#[tokio::test]
async fn composes_on_every_device() {
  let Some((engine, pack, dir)) = setup() else { return };
  for cpu_only in [false, true] {
    let output = dir.path().join("é.wav");
    let options = Options { cpu_only, ..Default::default() };
    let track = engine.compose(async { Ok(pack.clone()) }, &Request { options, ..request(&output) }, |_| {}).await.unwrap();
    assert_eq!(track.duration_ms, 10_000);
    // Exactly ten seconds of 16-bit stereo, and no leftover files.
    assert_eq!(std::fs::metadata(&output).unwrap().len(), 44 + 10 * 48_000 * 4);
    assert_eq!(std::fs::read_dir(dir.path()).unwrap().count(), 1);
  }
}

#[tokio::test]
async fn reports_missing_models() {
  let Some((engine, _, dir)) = setup() else { return };
  let (output, empty) = (dir.path().join("a.wav"), dir.path().to_path_buf());
  let result = engine.compose(async { Ok(empty) }, &request(&output), |_| {}).await;
  assert_eq!(result.unwrap_err(), MusicError::Model);
}

#[tokio::test]
async fn cancels_running_and_queued_requests() {
  let Some((engine, pack, dir)) = setup() else { return };
  let (first, second) = (dir.path().join("a.wav"), dir.path().join("b.wav"));
  let (first_request, second_request) = (request(&first), request(&second));
  let started = std::time::Instant::now();
  let (running, queued, ()) = tokio::join!(
    engine.compose(async { Ok(pack.clone()) }, &first_request, |_| {}),
    engine.compose(async { Ok(pack.clone()) }, &second_request, |_| {}),
    async {
      tokio::time::sleep(std::time::Duration::from_secs(3)).await;
      engine.cancel();
    }
  );
  assert_eq!(running.unwrap_err(), MusicError::Cancelled);
  assert_eq!(queued.unwrap_err(), MusicError::Cancelled);
  assert!(started.elapsed().as_secs() < 5);
  let _suspended = engine.suspend().await;
  assert_eq!(std::fs::read_dir(dir.path()).unwrap().count(), 0);
}
