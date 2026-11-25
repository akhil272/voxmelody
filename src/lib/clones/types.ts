export type CloneConsent = "own" | "permission";

export type CloneError =
  | "tooShort"
  | "tooQuiet"
  | "audio"
  | "microphone"
  | "consent"
  | "name"
  | "missing"
  | "storage";

export interface ClonedVoice {
  id: string;
  name: string;
  consent: CloneConsent;
  createdMs: number;
  durationMs: number;
  peaks: number[];
}

/** A checked recording that is not saved yet. */
export interface Sample {
  durationMs: number;
  peaks: number[];
}
