export type MusicError = "notInstalled" | "model" | "output" | "request" | "engine" | "cancelled";
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
  options: MusicOptions;
}
