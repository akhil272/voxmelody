//! What an assistant can do. Cloned voices are left out on purpose, so speech
//! in a person's voice is only ever made by someone using the app.

use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::{Value, json};
use tauri::{AppHandle, Manager};

use crate::library::store::{Library, Recording};
use crate::mix::commands::MixRequest;
use crate::models::manager::{ModelManager, PackState};
use crate::models::manifest::{self, Pack, PackKind};
use crate::music::commands::MusicRequest;
use crate::preferences;
use crate::takes::Take;
use crate::voice::commands::SpeechRequest;
use crate::voice::voices;

pub const SPEECH: &str = "generate_speech";
pub const MUSIC: &str = "generate_music";
const MIX: &str = "create_mix";
const VOICES: &str = "list_voices";
const RECORDINGS: &str = "list_recordings";
const MAX_RECORDINGS: usize = 50;

pub fn definitions() -> Vec<Value> {
  let tool = |name: &str, description: &str, properties: Value, required: &[&str]| {
    json!({
      "name": name,
      "description": description,
      "inputSchema": { "type": "object", "properties": properties, "required": required, "additionalProperties": false }
    })
  };
  vec![
    tool(
      VOICES,
      "Lists the voices that are ready to use.",
      json!({ "language": { "type": "string", "description": "Only this language, such as en-us or fr" } }),
      &[],
    ),
    tool(
      SPEECH,
      "Turns text into speech and saves it to the library as a WAV file.",
      json!({
        "text": { "type": "string", "maxLength": 10_000 },
        "voice": { "type": "string", "description": "A voice id from list_voices; omit for the default" },
        "model": { "type": "string", "description": "The model of that voice; omit for the default" },
        "speed": { "type": "number", "minimum": 0.5, "maximum": 1.5, "default": 1 }
      }),
      &["text"],
    ),
    tool(
      MUSIC,
      "Creates an instrumental track and saves it to the library as a WAV file. This can take minutes.",
      json!({
        "prompt": { "type": "string", "maxLength": 500, "description": "Style, mood and instruments" },
        "seconds": { "type": "integer", "minimum": 10, "maximum": 240, "default": 30 },
        "bpm": { "type": "integer", "minimum": 30, "maximum": 300 },
        "key": { "type": "string", "description": "Such as C major or F# minor" }
      }),
      &["prompt"],
    ),
    tool(
      MIX,
      "Combines a voice recording with a music recording and saves the mix as a WAV file.",
      json!({
        "voice": { "type": "integer", "description": "Recording id of kind voice" },
        "music": { "type": "integer", "description": "Recording id of kind music" },
        "volume": { "type": "number", "minimum": 0, "maximum": 1, "default": 0.5 },
        "intro": { "type": "number", "minimum": 0, "maximum": 10, "default": 2 },
        "outro": { "type": "number", "minimum": 1, "maximum": 10, "default": 3 }
      }),
      &["voice", "music"],
    ),
    tool(RECORDINGS, "Lists the newest recordings in the library.", json!({}), &[]),
  ]
}

pub fn exists(name: &str) -> bool {
  [VOICES, SPEECH, MUSIC, MIX, RECORDINGS].contains(&name)
}

/// Explains an error code.
fn explain(code: impl Serialize) -> String {
  let code = serde_json::to_value(code).ok().and_then(|value| value.as_str().map(String::from)).unwrap_or_default();
  let reason = match code.as_str() {
    "notInstalled" => "Download a model in VoxMelody first.",
    "noSpeech" => "The text has nothing to speak.",
    "tooLong" => "The text is too long.",
    "missing" => "The recording was not found, or is of another kind.",
    "request" => "Check the arguments: one of them is not supported.",
    "cancelled" => "Cancelled.",
    "output" | "diskFull" | "storage" => "The recording could not be saved.",
    _ => "Generation failed.",
  };
  format!("{reason} ({code})")
}

/// Fills in what the assistant left out. Only documented arguments count, and
/// their values are checked by the same commands the app uses.
fn arguments<T: DeserializeOwned>(tool: &str, mut defaults: Value, given: Value) -> Result<T, String> {
  let documented = definitions().into_iter().find(|definition| definition["name"] == tool).unwrap_or_default();
  let is_documented = |name: &str| documented["inputSchema"]["properties"].get(name).is_some();
  if let (Some(values), Value::Object(given)) = (defaults.as_object_mut(), given) {
    values.extend(given.into_iter().filter(|(name, value)| is_documented(name) && !value.is_null()));
  }
  serde_json::from_value(defaults).map_err(|error| format!("Check the arguments: {error}"))
}

async fn installed(app: &AppHandle, kind: PackKind) -> Vec<&'static Pack> {
  let models = app.state::<ModelManager>();
  let mut packs = Vec::new();
  for pack in manifest::packs().iter().filter(|pack| pack.kind == kind && !voices::can_clone(pack)) {
    if models.status(pack).await.state == PackState::Installed {
      packs.push(pack);
    }
  }
  packs
}

/// The model chosen in the app, or the first that is ready.
async fn preferred(app: &AppHandle, kind: PackKind) -> Result<&'static Pack, String> {
  let packs = installed(app, kind).await;
  let chosen = preferences::model(app, kind);
  let pack = packs.iter().find(|pack| pack.id == chosen).or(packs.first());
  pack.copied().ok_or_else(|| explain("notInstalled"))
}

fn saved(app: &AppHandle, take: Take) -> Result<Value, String> {
  let (path, title) = app.state::<Library>().open(take.id).map_err(explain)?;
  Ok(json!({ "id": take.id, "title": title, "durationMs": take.duration_ms, "path": path }))
}

async fn list_voices(app: &AppHandle, given: &Value) -> Vec<voices::Voice> {
  let language = given["language"].as_str();
  let packs = installed(app, PackKind::Voice).await;
  let all = packs.into_iter().flat_map(|pack| voices::of(pack));
  all.filter(|voice| language.is_none_or(|language| voice.language == language)).collect()
}

async fn speech(app: &AppHandle, given: Value) -> Result<Value, String> {
  let model = match given["model"].as_str().map(manifest::find) {
    Some(model) => model.filter(|model| !voices::can_clone(model)).ok_or_else(|| explain("request"))?,
    None => preferred(app, PackKind::Voice).await?,
  };
  let voice = voices::of(model).first().map(|voice| voice.id.clone()).unwrap_or_default();
  let defaults = json!({
    "text": "", "model": model.id, "voice": voice, "speed": 1, "preview": false, "pause": 1, "threads": 0, "steps": 0
  });
  let request: SpeechRequest = arguments(SPEECH, defaults, given)?;
  let take = crate::voice::commands::speak(app.clone(), app.state(), app.state(), app.state(), app.state(), request);
  saved(app, take.await.map_err(explain)?)
}

async fn music(app: &AppHandle, given: Value) -> Result<Value, String> {
  let model = preferred(app, PackKind::Music).await?;
  let defaults = json!({
    "model": model.id, "prompt": "", "seconds": 30, "bpm": 0, "key": "",
    "options": { "seed": null, "steps": 0, "guidance": 0, "creativity": 0, "cpuOnly": false }
  });
  let request: MusicRequest = arguments(MUSIC, defaults, given)?;
  let take = crate::music::commands::compose(app.clone(), app.state(), app.state(), app.state(), request);
  saved(app, take.await.map_err(explain)?)
}

async fn mix(app: &AppHandle, given: Value) -> Result<Value, String> {
  let request: MixRequest = arguments(MIX, json!({ "volume": 0.5, "intro": 2, "outro": 3 }), given)?;
  let take = crate::mix::commands::create_mix(app.clone(), app.state(), app.state(), request);
  saved(app, take.await.map_err(explain)?)
}

fn recordings(app: &AppHandle) -> Result<Value, String> {
  let library = app.state::<Library>();
  let describe = |recording: Recording| {
    let path = library.open(recording.id).ok().map(|(path, _)| path);
    json!({
      "id": recording.id, "kind": recording.kind, "title": recording.title,
      "durationMs": recording.duration_ms, "path": path
    })
  };
  Ok(library.list().map_err(explain)?.into_iter().take(MAX_RECORDINGS).map(describe).collect())
}

pub async fn call(app: &AppHandle, name: &str, given: Value) -> Result<Value, String> {
  match name {
    VOICES => Ok(json!(list_voices(app, &given).await)),
    SPEECH => speech(app, given).await,
    MUSIC => music(app, given).await,
    MIX => mix(app, given).await,
    _ => recordings(app),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn fills_in_defaults() {
    let defaults = || json!({ "volume": 0.5, "intro": 2, "outro": 3 });
    assert!(arguments::<MixRequest>(MIX, defaults(), json!({ "voice": 1, "music": 2, "volume": null })).is_ok());
    assert!(arguments::<MixRequest>(MIX, defaults(), json!({ "voice": 1 })).is_err());
    assert!(arguments::<MixRequest>(MIX, defaults(), json!({ "voice": "x", "music": 2 })).is_err());
  }

  #[test]
  fn ignores_undocumented_arguments() {
    let given = json!({ "text": "Hi", "preview": true, "reference": "x" });
    let defaults = json!({
      "text": "", "model": "m", "voice": "v", "speed": 1, "preview": false, "pause": 1, "threads": 0, "steps": 0
    });
    let request: SpeechRequest = arguments(SPEECH, defaults, given).unwrap();
    assert!(!request.preview && request.text == "Hi");
  }

  #[test]
  fn explains_codes() {
    assert_eq!(explain(crate::voice::error::VoiceError::NoSpeech), "The text has nothing to speak. (noSpeech)");
    assert_eq!(explain("engine"), "Generation failed. (engine)");
  }
}
