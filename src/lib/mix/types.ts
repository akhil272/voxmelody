export type MixError = "missing" | "audio" | "diskFull" | "output" | "request";

export interface MixRequest {
  voice: number;
  music: number;
  /** From 0 to 1. */
  volume: number;
  intro: number;
  outro: number;
}
