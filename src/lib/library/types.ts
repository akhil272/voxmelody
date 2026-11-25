import type { PackId } from "#lib/models/types.js";

export type RecordingKind = PackId | "mix";
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
