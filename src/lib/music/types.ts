export type MusicError = "notInstalled" | "model" | "output" | "request" | "engine" | "cancelled";
export type TempoMode = "auto" | "custom";

export interface MusicRequest {
  prompt: string;
  seconds: number;
  bpm: number;
  key: string;
}
