use crate::engine::Language;

/// Abbreviations whose full stop would otherwise end the sentence.
const ENGLISH_ABBREVIATIONS: [(&str, &str); 10] = [
  ("Dr.", "Doctor"),
  ("Mr.", "Mister"),
  ("Mrs.", "Missus"),
  ("Ms.", "Miz"),
  ("Prof.", "Professor"),
  ("vs.", "versus"),
  ("etc.", "et cetera"),
  ("e.g.", "for example"),
  ("i.e.", "that is"),
  ("approx.", "approximately"),
];

/// Cleans text so the speech library reads every word.
pub fn prepare(text: &str, language: Language) -> String {
  let clean: String = text
    .chars()
    .map(|c| match c {
      '\u{2018}' | '\u{2019}' => '\'',
      '\u{201c}' | '\u{201d}' => '"',
      '\u{2013}' | '\u{2014}' => ',',
      '\n' => '\n',
      c if c.is_control() || c.is_whitespace() => ' ',
      c => c,
    })
    .collect();
  clean
    .lines()
    .map(|line| prepare_line(line, language))
    .filter(|line| !line.is_empty())
    .collect::<Vec<_>>()
    .join(" ")
}

fn prepare_line(line: &str, language: Language) -> String {
  let words: Vec<&str> = line.split_whitespace().collect();
  let mut spoken: Vec<String> = Vec::with_capacity(words.len());
  for (index, word) in words.iter().enumerate() {
    spoken.push(if language.is_english() { expand(word, index + 1 == words.len()) } else { (*word).into() });
  }
  let mut line = spoken.join(" ");
  // Keeps headings and list items from running into the next line.
  if line.chars().next_back().is_some_and(char::is_alphanumeric) {
    line.push('.');
  }
  line
}

fn expand(word: &str, ends_line: bool) -> String {
  let start = word.find(char::is_alphanumeric).unwrap_or(0);
  let (prefix, core) = word.split_at(start);
  for (short, long) in ENGLISH_ABBREVIATIONS {
    if let Some(rest) = core.strip_prefix(short).filter(|rest| !rest.starts_with(char::is_alphanumeric)) {
      let stop = if ends_line && rest.is_empty() { "." } else { "" };
      return format!("{prefix}{long}{rest}{stop}");
    }
  }
  word.into()
}

#[cfg(test)]
mod tests {
  use super::*;

  fn english(text: &str) -> String {
    prepare(text, Language::EnUs)
  }

  #[test]
  fn expands_abbreviations() {
    assert_eq!(english("Dr. Smith (Mr. Jones) vs. cats, etc."), "Doctor Smith (Mister Jones) versus cats, et cetera.");
    assert_eq!(english("e.g., tea"), "for example, tea.");
    assert_eq!(english("The end etc.\nNext"), "The end et cetera. Next.");
  }

  #[test]
  fn leaves_other_words_and_languages_alone() {
    assert_eq!(english("Mr.Big Drive. $49.99"), "Mr.Big Drive. $49.99.");
    assert_eq!(prepare("El Dr. Niño", Language::Es), "El Dr. Niño.");
  }

  #[test]
  fn removes_control_characters_and_smart_punctuation() {
    assert_eq!(english("it\u{2019}s\0 \u{201c}fine\u{201d}\t\r\nok!"), "it's \"fine\" ok!");
    assert_eq!(english(" \n\0 "), "");
  }

  #[test]
  fn separates_headings() {
    assert_eq!(english("Chapter One\n\nIt began"), "Chapter One. It began.");
  }
}
