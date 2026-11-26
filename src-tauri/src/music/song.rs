//! What a song needs beyond an instrumental.

use serde::Deserialize;

const MAX_CHARACTERS: usize = 2_500;
pub const SECONDS: std::ops::RangeInclusive<u32> = 20..=240;
/// Room for an opening and an ending.
const EXTRA_SECONDS: f32 = 14.0;
const LINE_SECONDS: f32 = 4.0;
const CHARACTER_SECONDS: f32 = 0.15;
/// Languages whose words were recognised in test songs.
pub const LANGUAGES: [&str; 9] = ["en", "de", "es", "fr", "it", "ko", "pl", "pt", "tr"];

#[derive(Debug, Deserialize)]
pub struct SongRequest {
  pub lyrics: String,
  /// Empty lets the model tell.
  pub language: String,
}

/// Lines that are sung; section names such as `[Chorus]` are not.
fn sung(lyrics: &str) -> impl Iterator<Item = &str> {
  lyrics.lines().map(str::trim).filter(|line| line.chars().any(char::is_alphanumeric) && !line.starts_with('['))
}

impl SongRequest {
  pub fn is_valid(&self) -> bool {
    sung(&self.lyrics).next().is_some()
      && self.lyrics.chars().count() <= MAX_CHARACTERS
      && (self.language.is_empty() || LANGUAGES.contains(&self.language.as_str()))
  }

  /// A length that fits the lyrics.
  pub fn seconds(&self) -> u32 {
    let line = |line: &str| (line.chars().count() as f32 * CHARACTER_SECONDS).max(LINE_SECONDS);
    let total = EXTRA_SECONDS + sung(&self.lyrics).map(line).sum::<f32>();
    (total as u32).clamp(*SECONDS.start(), *SECONDS.end())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn song(lyrics: &str, language: &str) -> SongRequest {
    SongRequest { lyrics: lyrics.into(), language: language.into() }
  }

  #[test]
  fn checks_lyrics_and_language() {
    assert!(song("[Verse]\nMorning light", "en").is_valid());
    assert!(song("朝の光", "").is_valid());
    for lyrics in ["", " \n ", "[Instrumental]", "[Verse]\n...", "[Chorus]\n\n[Verse]"] {
      assert!(!song(lyrics, "en").is_valid(), "{lyrics}");
    }
    assert!(!song("Hello", "xx").is_valid());
    assert!(song(&"é".repeat(2_500), "").is_valid());
    assert!(!song(&"é".repeat(2_501), "").is_valid());
  }

  #[test]
  fn fits_the_length_to_the_lyrics() {
    assert_eq!(song("Hello", "").seconds(), *SECONDS.start());
    assert_eq!(song(&"Little boats are coming home\n".repeat(14), "").seconds(), 72);
    assert_eq!(song(&"A line\n".repeat(200), "").seconds(), *SECONDS.end());
  }
}
