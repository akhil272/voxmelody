use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Voice {
  pub id: &'static str,
  pub name: &'static str,
  pub language: &'static str,
  #[serde(skip)]
  pub speaker: i32,
}

const fn voice(speaker: i32, id: &'static str, name: &'static str, language: &'static str) -> Voice {
  Voice { id, name, language, speaker }
}

/// Speakers in the Kokoro 1.0 pack, best default first within each language.
pub const VOICES: [Voice; 41] = [
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

pub fn find(id: &str) -> Option<&'static Voice> {
  VOICES.iter().find(|voice| voice.id == id)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn ids_and_speakers_are_unique() {
    let ids: std::collections::HashSet<_> = VOICES.iter().map(|voice| voice.id).collect();
    let speakers: std::collections::HashSet<_> = VOICES.iter().map(|voice| voice.speaker).collect();
    assert_eq!((ids.len(), speakers.len()), (VOICES.len(), VOICES.len()));
  }

  #[test]
  fn ids_match_languages() {
    for voice in &VOICES {
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
