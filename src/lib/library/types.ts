import type { ModelKind } from "#lib/models/types.js";

export type RecordingKind = ModelKind | "song" | "mix";
export type LibraryError = "missing" | "mismatch" | "diskFull" | "storage";

export interface Recording {
  id: number;
  kind: RecordingKind;
  title: string;
  durationMs: number;
  peaks: number[];
  createdMs: number;
  /** True when it contains a cloned voice. */
  cloned: boolean;
  available: boolean;
}

export type LibraryFilter = "all" | "voice" | "music" | "mix";

export interface SaveFolder {
  path: string;
  isDefault: boolean;
  /** False when the chosen folder cannot be reached. */
  available: boolean;
}
