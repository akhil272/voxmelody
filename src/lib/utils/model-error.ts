import type { ModelError } from "#lib/models/types.js";

const MODEL_ERRORS: readonly string[] = ["unknownPack", "network", "corrupt", "diskFull", "storage", "locationMissing"];

export function toModelError(error: unknown): ModelError {
  return typeof error === "string" && MODEL_ERRORS.includes(error) ? (error as ModelError) : "storage";
}
