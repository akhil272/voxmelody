import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import type { Take } from "#lib/takes/types.js";
import type { SpeechRequest, Voice } from "./types.js";

export function listVoices() {
  return invoke<Voice[]>("list_voices");
}

export function speak(request: SpeechRequest) {
  return invoke<Take>("speak", { request });
}

export function cancelSpeech() {
  return invoke<void>("cancel_speech");
}

export function onProgress(handler: (value: number) => void) {
  return listen<number>("voice://progress", (event) => handler(event.payload));
}
