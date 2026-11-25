import { invoke } from "@tauri-apps/api/core";
import type { Take } from "#lib/takes/types.js";
import type { MixRequest } from "./types.js";

export function createMix(request: MixRequest) {
  return invoke<Take>("create_mix", { request });
}
