import type { CloneError } from "#lib/clones/types.js";

const CLONE_ERRORS: readonly string[] = [
  "tooShort",
  "tooQuiet",
  "audio",
  "microphone",
  "consent",
  "name",
  "missing",
  "storage",
];

export function toCloneError(error: unknown): CloneError {
  const code = error instanceof Error ? error.message : error;
  return typeof code === "string" && CLONE_ERRORS.includes(code) ? (code as CloneError) : "storage";
}
