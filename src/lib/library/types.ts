import type { PackId } from "#lib/models/types.js";

export type LibraryError = "missing" | "mismatch" | "diskFull" | "storage";

export interface Recording {
  id: number;
  kind: PackId;
  title: string;
  durationMs: number;
  peaks: number[];
  createdMs: number;
  available: boolean;
}
