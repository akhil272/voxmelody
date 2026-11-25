import type { TempoMode } from "#lib/music/types.js";
import { COPY } from "./copy.js";

export const MAX_PROMPT_LENGTH = 500;
export const DEFAULT_LENGTH = 30;
export const LENGTH = { min: 10, max: 240, step: 5 };
export const AUTO_BPM = 0;
export const DEFAULT_BPM = 120;
export const BPM = { min: 60, max: 200, step: 1 };

export const TEMPO_MODES: { value: TempoMode; label: string }[] = [
  { value: "auto", label: COPY.music.auto },
  { value: "custom", label: COPY.music.custom },
];

/** Select values cannot be empty. */
export const AUTO_KEY = "auto";
const NOTES = ["C", "C#", "D", "Eb", "E", "F", "F#", "G", "Ab", "A", "Bb", "B"];
const MODES = ["major", "minor"];

export const MUSIC_KEYS = [
  { value: AUTO_KEY, label: COPY.music.auto },
  ...NOTES.flatMap((note) =>
    MODES.map((mode) => ({
      value: `${note} ${mode}`,
      label: `${note.replace("#", "♯").replace("b", "♭")} ${mode}`,
    })),
  ),
];
