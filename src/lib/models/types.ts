export type ModelKind = "voice" | "music";
export type PackState = "available" | "downloading" | "paused" | "installed";
export type ModelError = "unknownPack" | "network" | "corrupt" | "diskFull" | "storage" | "locationMissing" | "paused";

export interface PackStatus {
  id: string;
  state: PackState;
  downloadedBytes: number;
  totalBytes: number;
  downloadBytes: number;
  error: ModelError | null;
}

export interface VoiceModel {
  family: "kokoro" | "kitten" | "supertonic";
}

export interface MusicModel {
  steps: number;
  guidance: boolean;
}

export interface ModelInfo {
  id: string;
  kind: ModelKind;
  name: string;
  author: string;
  /** As YYYY-MM-DD. */
  released: string;
  licence: string;
  restricted: boolean;
  pageUrl: string;
  parameters: string;
  precision: string;
  languages: string[];
  recommendedMemory: number;
  voice: VoiceModel | null;
  music: MusicModel | null;
}

export type ModelSize = "any" | "small" | "medium" | "large";
export type ModelSort = "recommended" | "newest" | "smallest" | "name";

export interface ModelFilter {
  search: string;
  kind: ModelKind | "all";
  licence: string;
  size: ModelSize;
  sort: ModelSort;
  favourites: boolean;
}

export interface LocationStatus {
  path: string;
  isDefault: boolean;
  available: boolean;
  movePercent: number | null;
}

export interface SystemInfo {
  memoryBytes: number;
  freeDiskBytes: number | null;
  cpuThreads: number;
}
