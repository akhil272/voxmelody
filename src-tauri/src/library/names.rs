use std::path::{Path, PathBuf};

const TITLE_LENGTH: usize = 80;
const FILE_LENGTH: usize = 40;
const FALLBACK: &str = "VoxMelody";
/// Windows refuses these as file names, whatever the extension.
const RESERVED: [&str; 4] = ["CON", "PRN", "AUX", "NUL"];
const RESERVED_PORTS: [&str; 2] = ["COM", "LPT"];

fn shorten(text: &str, length: usize) -> String {
  text.chars().take(length).collect::<String>().trim().to_string()
}

/// One line of at most 80 characters, or `None` when nothing is left.
pub fn title(text: &str) -> Option<String> {
  let line = text.split_whitespace().collect::<Vec<_>>().join(" ");
  Some(shorten(&line, TITLE_LENGTH)).filter(|title| !title.is_empty())
}

fn is_reserved(stem: &str) -> bool {
  let upper = stem.to_uppercase();
  let is_port = upper.len() == 4
    && RESERVED_PORTS.iter().any(|port| upper.starts_with(port))
    && upper.ends_with(|last: char| last.is_ascii_digit());
  is_port || RESERVED.contains(&upper.as_str())
}

/// A file name stem that is valid on every system.
fn stem(title: &str) -> String {
  // Dots go too: a title must not look like an extension or a parent folder.
  let unsafe_character = |c: char| c.is_control() || r#"<>:"/\|?*."#.contains(c);
  let clean: String = title.chars().map(|c| if unsafe_character(c) { ' ' } else { c }).collect();
  let stem = shorten(&clean.split_whitespace().collect::<Vec<_>>().join(" "), FILE_LENGTH);
  if stem.is_empty() || is_reserved(&stem) { FALLBACK.into() } else { stem }
}

/// Creates a new empty file, adding a number when the name is taken.
pub fn reserve(dir: &Path, title: &str) -> std::io::Result<PathBuf> {
  let stem = stem(title);
  for number in 1u32.. {
    let name = if number == 1 { format!("{stem}.wav") } else { format!("{stem} {number}.wav") };
    let path = dir.join(name);
    match std::fs::File::create_new(&path) {
      Ok(_) => return Ok(path),
      Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
      Err(error) => return Err(error),
    }
  }
  Err(std::io::ErrorKind::AlreadyExists.into())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn titles_are_one_short_line() {
    assert_eq!(title("  Hello\n\tworld ").unwrap(), "Hello world");
    assert_eq!(title(&"é".repeat(200)).unwrap().chars().count(), TITLE_LENGTH);
    assert!(title(" \n ").is_none());
  }

  #[test]
  fn stems_are_safe() {
    assert_eq!(stem("../..\\a:b?"), "a b");
    assert_eq!(stem("नमस्ते दुनिया"), "नमस्ते दुनिया");
    for name in ["...", "con", "LPT1"] {
      assert_eq!(stem(name), FALLBACK);
    }
    assert_eq!(stem("Comet"), "Comet");
  }

  #[test]
  fn never_overwrites() {
    let dir = tempfile::tempdir().unwrap();
    let names: Vec<_> = (0..3)
      .map(|_| reserve(dir.path(), "Take").unwrap().file_name().unwrap().to_string_lossy().into_owned())
      .collect();
    assert_eq!(names, ["Take.wav", "Take 2.wav", "Take 3.wav"]);
  }
}
