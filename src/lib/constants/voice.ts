import type { VoiceLanguage } from "#lib/voice/types.js";

export const MAX_SCRIPT_LENGTH = 10_000;
export const DEFAULT_SPEED = 1;
export const SPEED = { min: 0.5, max: 1.5, step: 0.05 };

export const VOICE_LANGUAGES: { value: VoiceLanguage; label: string }[] = [
  { value: "en-us", label: "English (US)" },
  { value: "en-gb", label: "English (UK)" },
  { value: "es", label: "Spanish" },
  { value: "fr", label: "French" },
  { value: "hi", label: "Hindi" },
  { value: "it", label: "Italian" },
  { value: "pt-br", label: "Portuguese (Brazil)" },
];
