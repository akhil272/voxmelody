import type { Processor, TempoMode } from "#lib/music/types.js";
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

export const CREATIVITY = { min: 0.3, max: 1.3, step: 0.05 };
export const DEFAULT_CREATIVITY = 0.85;
export const MUSIC_STEPS = { min: 4, max: 100, step: 1 };
export const GUIDANCE = { min: 1, max: 15, step: 0.5 };
export const DEFAULT_GUIDANCE = 1;
export const MAX_SEED = 4294967295;

export const PROCESSORS: { value: Processor; label: string }[] = [
  { value: "auto", label: COPY.music.processors.auto },
  { value: "cpu", label: COPY.music.processors.cpu },
];
