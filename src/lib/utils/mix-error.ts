import type { MixError } from "#lib/mix/types.js";

const MIX_ERRORS: readonly string[] = ["missing", "audio", "diskFull", "output", "request"];

export function toMixError(error: unknown): MixError {
  return typeof error === "string" && MIX_ERRORS.includes(error) ? (error as MixError) : "output";
}
