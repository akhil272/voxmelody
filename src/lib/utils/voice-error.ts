import type { VoiceError } from "#lib/voice/types.js";

const VOICE_ERRORS: readonly string[] = [
  "notInstalled",
  "noSpeech",
  "tooLong",
  "model",
  "output",
  "reference",
  "request",
  "engine",
  "cancelled",
];

export function toVoiceError(error: unknown): VoiceError {
  return typeof error === "string" && VOICE_ERRORS.includes(error) ? (error as VoiceError) : "engine";
}
