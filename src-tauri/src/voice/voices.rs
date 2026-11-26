use serde::Serialize;

use crate::models::manifest::PackInfo;

/// Ids are unique within a model.
#[derive(Debug, Clone, Serialize)]
pub struct Voice {
  pub id: String,
  pub model: String,
  pub name: String,
  pub language: String,
  #[serde(skip)]
  pub speaker: i32,
}

/// The family that speaks in the voice of a recording.
const CLONING: &str = "pocket";

struct Speaker {
  id: &'static str,
  name: &'static str,
  language: &'static str,
  number: i32,
}

const fn voice(number: i32, id: &'static str, name: &'static str, language: &'static str) -> Speaker {
  Speaker { id, name, language, number }
}

/// Kokoro 1.0, best default first within each language.
const KOKORO: [Speaker; 41] = [
  voice(3, "af_heart", "Heart", "en-us"),
  voice(0, "af_alloy", "Alloy", "en-us"),
  voice(1, "af_aoede", "Aoede", "en-us"),
  voice(2, "af_bella", "Bella", "en-us"),
  voice(4, "af_jessica", "Jessica", "en-us"),
  voice(5, "af_kore", "Kore", "en-us"),
  voice(6, "af_nicole", "Nicole", "en-us"),
  voice(7, "af_nova", "Nova", "en-us"),
  voice(8, "af_river", "River", "en-us"),
  voice(9, "af_sarah", "Sarah", "en-us"),
  voice(10, "af_sky", "Sky", "en-us"),
  voice(11, "am_adam", "Adam", "en-us"),
  voice(12, "am_echo", "Echo", "en-us"),
  voice(13, "am_eric", "Eric", "en-us"),
  voice(14, "am_fenrir", "Fenrir", "en-us"),
  voice(15, "am_liam", "Liam", "en-us"),
  voice(16, "am_michael", "Michael", "en-us"),
  voice(17, "am_onyx", "Onyx", "en-us"),
  voice(18, "am_puck", "Puck", "en-us"),
  voice(19, "am_santa", "Santa", "en-us"),
  voice(21, "bf_emma", "Emma", "en-gb"),
  voice(20, "bf_alice", "Alice", "en-gb"),
  voice(22, "bf_isabella", "Isabella", "en-gb"),
  voice(23, "bf_lily", "Lily", "en-gb"),
  voice(24, "bm_daniel", "Daniel", "en-gb"),
  voice(25, "bm_fable", "Fable", "en-gb"),
  voice(26, "bm_george", "George", "en-gb"),
  voice(27, "bm_lewis", "Lewis", "en-gb"),
  voice(28, "ef_dora", "Dora", "es"),
  voice(29, "em_alex", "Alex", "es"),
  voice(53, "em_santa", "Santa", "es"),
  voice(30, "ff_siwis", "Siwis", "fr"),
  voice(31, "hf_alpha", "Alpha", "hi"),
  voice(32, "hf_beta", "Beta", "hi"),
  voice(33, "hm_omega", "Omega", "hi"),
  voice(34, "hm_psi", "Psi", "hi"),
  voice(35, "if_sara", "Sara", "it"),
  voice(36, "im_nicola", "Nicola", "it"),
  voice(42, "pf_dora", "Dora", "pt-br"),
  voice(43, "pm_alex", "Alex", "pt-br"),
  voice(44, "pm_santa", "Santa", "pt-br"),
];

/// Kitten 0.8.
const KITTEN: [(i32, &str); 8] =
  [(1, "Bella"), (0, "Jasper"), (3, "Luna"), (2, "Bruno"), (5, "Rosie"), (4, "Hugo"), (7, "Kiki"), (6, "Leo")];

/// Supertonic 3 voices speak every language.
const SUPERTONIC: [(i32, &str); 10] = [
  (0, "Female 1"),
  (1, "Female 2"),
  (2, "Female 3"),
  (3, "Female 4"),
  (4, "Female 5"),
  (5, "Male 1"),
  (6, "Male 2"),
  (7, "Male 3"),
  (8, "Male 4"),
  (9, "Male 5"),
];

/// The voices of one model, the default first. Models for cloning have none of their own.
pub fn of(model: &PackInfo) -> Vec<Voice> {
  let voice = |id: String, name: &str, language: &str, speaker| Voice {
    id,
    model: model.id.clone(),
    name: name.into(),
    language: language.into(),
    speaker,
  };
  let every_language = |speakers: &[(i32, &'static str)]| {
    let spoken = |language: &String| {
      speakers.iter().map(|(number, name)| voice(format!("{language}-{number}"), name, language, *number)).collect()
    };
    model.languages.iter().flat_map(|language| -> Vec<Voice> { spoken(language) }).collect()
  };
  match model.voice.as_ref().map(|voice| voice.family.as_str()) {
    Some("kokoro") => KOKORO.iter().map(|s| voice(s.id.into(), s.name, s.language, s.number)).collect(),
    Some("kitten") => every_language(&KITTEN),
    Some("supertonic") => every_language(&SUPERTONIC),
    _ => Vec::new(),
  }
}

pub fn can_clone(model: &PackInfo) -> bool {
  model.voice.as_ref().is_some_and(|voice| voice.family == CLONING)
}

pub fn find(model: &PackInfo, id: &str) -> Option<Voice> {
  of(model).into_iter().find(|voice| voice.id == id)
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::models::manifest;
  use std::collections::HashSet;

  #[test]
  fn every_voice_model_has_voices_for_its_languages() {
    for pack in manifest::packs().iter().filter(|pack| pack.voice.is_some() && !can_clone(pack)) {
      let voices = of(pack);
      let ids: HashSet<_> = voices.iter().map(|voice| &voice.id).collect();
      assert_eq!(ids.len(), voices.len(), "{}", pack.id);
      let spoken: HashSet<_> = voices.iter().map(|voice| &voice.language).collect();
      assert_eq!(spoken, pack.languages.iter().collect(), "{}", pack.id);
      let speakers: HashSet<_> = voices.iter().map(|voice| (&voice.language, voice.speaker)).collect();
      assert_eq!(speakers.len(), voices.len(), "{}", pack.id);
    }
  }

  #[test]
  fn kokoro_ids_match_languages() {
    for voice in &KOKORO {
      let expected = match &voice.id[..1] {
        "a" => "en-us",
        "b" => "en-gb",
        "e" => "es",
        "f" => "fr",
        "h" => "hi",
        "i" => "it",
        _ => "pt-br",
      };
      assert_eq!(voice.language, expected, "{}", voice.id);
    }
  }
}
