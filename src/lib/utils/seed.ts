import { MAX_SEED } from "#lib/constants.js";

/** Null means a new result every time. */
export function parseSeed(text: string) {
  const value = text.trim();
  return /^\d+$/.test(value) && Number(value) <= MAX_SEED ? Number(value) : null;
}

export function isSeed(text: string) {
  return text.trim() === "" || parseSeed(text) !== null;
}
