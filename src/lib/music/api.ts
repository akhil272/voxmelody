import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import type { Take } from "#lib/takes/types.js";
import type { MusicRequest } from "./types.js";

export function compose(request: MusicRequest) {
  return invoke<Take>("compose", { request });
}

export function cancelMusic() {
  return invoke<void>("cancel_music");
}

export function onProgress(handler: (value: number) => void) {
  return listen<number>("music://progress", (event) => handler(event.payload));
}
