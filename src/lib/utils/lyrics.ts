import { SECTION_TAGS } from "#lib/constants.js";
import type { SongSection } from "#lib/music/types.js";

/** Starts a new section at the end. */
export function addSection(lyrics: string, section: SongSection) {
  const before = lyrics.trimEnd();
  return `${before}${before && "\n\n"}${SECTION_TAGS[section]}\n`;
}

/** True when there is something to sing besides section names. */
export function hasLyrics(lyrics: string) {
  return lyrics.split("\n").some((line) => /[\p{L}\p{N}]/u.test(line) && !line.trim().startsWith("["));
}
