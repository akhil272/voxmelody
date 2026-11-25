export type PackId = "voice" | "music";
export type PackState = "available" | "downloading" | "paused" | "installed";
export type ModelError = "unknownPack" | "network" | "corrupt" | "diskFull" | "storage" | "locationMissing" | "paused";

export interface PackStatus {
  id: PackId;
  state: PackState;
  downloadedBytes: number;
  totalBytes: number;
  recommendedMemory: number;
  error: ModelError | null;
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
