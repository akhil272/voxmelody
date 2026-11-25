import { LANGUAGE_LABELS } from "#lib/constants.js";
import type { Voice } from "#lib/voice/types.js";

function baseLanguage(language: string) {
  return language.split("-")[0];
}

/** The chosen voice, or the closest one another model has. */
export function closestVoice(voices: Voice[], choice: Voice | null): Voice | undefined {
  if (!choice) return voices[0];
  const speaking = voices.filter(({ language }) => language === choice.language);
  return (
    voices.find(({ id, model }) => id === choice.id && model === choice.model) ??
    speaking.find(({ name }) => name === choice.name) ??
    speaking[0] ??
    voices.find(({ language }) => baseLanguage(language) === baseLanguage(choice.language)) ??
    voices[0]
  );
}

export function languageLabel(language: string) {
  return LANGUAGE_LABELS[language] ?? language;
}

/** Each language once, by name. */
export function toLanguageOptions(voices: Voice[]) {
  return [...new Set(voices.map(({ language }) => language))]
    .map((language) => ({ value: language, label: languageLabel(language) }))
    .sort((a, b) => a.label.localeCompare(b.label));
}

/** A few names, or a count when there are many. */
export function summariseLanguages(languages: string[]) {
  const names = new Set(languages.map((language) => languageLabel(language).replace(/ \(.*\)$/, "")));
  return names.size === 0 ? "" : names.size === 1 ? [...names][0] : String(names.size);
}
