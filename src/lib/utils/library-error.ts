import type { LibraryError } from "#lib/library/types.js";

const LIBRARY_ERRORS: readonly string[] = ["missing", "mismatch", "diskFull", "storage"];

export function toLibraryError(error: unknown): LibraryError {
  return typeof error === "string" && LIBRARY_ERRORS.includes(error) ? (error as LibraryError) : "storage";
}
