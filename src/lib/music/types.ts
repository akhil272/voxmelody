export type MusicError = "notInstalled" | "model" | "output" | "request" | "engine" | "cancelled";
export type MusicMode = "instrumental" | "song";
export type LengthMode = "auto" | "custom";
export type SongSection = "verse" | "chorus" | "bridge";
export type TempoMode = "auto" | "custom";

export type Processor = "auto" | "cpu";

/** Null or zero keeps the model's own choice. */
export interface MusicOptions {
  seed: number | null;
  steps: number;
  guidance: number;
  creativity: number;
  cpuOnly: boolean;
}

export interface MusicRequest {
  model: string;
  prompt: string;
  seconds: number;
  bpm: number;
  key: string;
  song: { lyrics: string; language: string } | null;
  options: MusicOptions;
}
