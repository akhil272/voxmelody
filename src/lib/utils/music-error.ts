import type { MusicError } from "#lib/music/types.js";

const MUSIC_ERRORS: readonly string[] = ["notInstalled", "model", "output", "request", "engine", "cancelled"];

export function toMusicError(error: unknown): MusicError {
  return typeof error === "string" && MUSIC_ERRORS.includes(error) ? (error as MusicError) : "engine";
}
