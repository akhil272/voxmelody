import type { ModelKind } from "#lib/models/types.js";

export type RecordingKind = ModelKind | "mix";
export type LibraryError = "missing" | "mismatch" | "diskFull" | "storage";

export interface Recording {
  id: number;
  kind: RecordingKind;
  title: string;
  durationMs: number;
  peaks: number[];
  createdMs: number;
  available: boolean;
}
