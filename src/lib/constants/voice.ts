export const MAX_SCRIPT_LENGTH = 10_000;
export const DEFAULT_SPEED = 1;
export const SPEED = { min: 0.5, max: 1.5, step: 0.05 };

export const DEFAULT_PAUSE = 1;
export const PAUSE = { min: 1, max: 5, step: 0.5 };
/** Zero lets the app choose. */
export const AUTO = 0;
export const VOICE_STEPS = { min: 0, max: 16, step: 1 };

export const LANGUAGE_LABELS: Record<string, string> = {
  "en-us": "English (US)",
  "en-gb": "English (UK)",
  en: "English",
  ar: "Arabic",
  bg: "Bulgarian",
  cs: "Czech",
  da: "Danish",
  de: "German",
  el: "Greek",
  es: "Spanish",
  et: "Estonian",
  fi: "Finnish",
  fr: "French",
  hi: "Hindi",
  hr: "Croatian",
  hu: "Hungarian",
  id: "Indonesian",
  it: "Italian",
  ja: "Japanese",
  ko: "Korean",
  lt: "Lithuanian",
  lv: "Latvian",
  nl: "Dutch",
  pl: "Polish",
  pt: "Portuguese",
  "pt-br": "Portuguese (Brazil)",
  ro: "Romanian",
  ru: "Russian",
  sk: "Slovak",
  sl: "Slovenian",
  sv: "Swedish",
  tr: "Turkish",
  uk: "Ukrainian",
  vi: "Vietnamese",
};
