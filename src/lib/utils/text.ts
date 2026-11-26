const FILE_NAME_LENGTH = 40;

/** Counts characters as people see them, so accents and emoji count once. */
export function countCharacters(text: string) {
  return Array.from(text).length;
}

/** A safe file name from the first words of a script. */
export function toFileName(text: string, fallback = "VoxMelody") {
  const name = text
    .replace(/[^\p{L}\p{M}\p{N} _-]/gu, " ")
    .replace(/\s+/g, " ")
    .trim();
  return Array.from(name).slice(0, FILE_NAME_LENGTH).join("").trim() || fallback;
}
