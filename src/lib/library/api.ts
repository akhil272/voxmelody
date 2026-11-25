import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { toAudioUrl, toFileName } from "#lib/utils.js";
import type { Recording } from "./types.js";

export function listRecordings() {
  return invoke<Recording[]>("list_recordings");
}

/** A playable address. Revoke it after use. */
export async function loadRecording(id: number) {
  return toAudioUrl(await invoke<ArrayBuffer>("read_recording", { id }));
}

export function renameRecording(id: number, title: string) {
  return invoke<void>("rename_recording", { id, title });
}

export function deleteRecording(id: number) {
  return invoke<void>("delete_recording", { id });
}

/** Resolves to false when the user closes the save dialog. */
export function exportRecording(id: number, title: string) {
  return invoke<boolean>("export_recording", { id, name: `${toFileName(title)}.wav` });
}

/** Resolves to false when the user closes the file picker. */
export function relinkRecording(id: number) {
  return invoke<boolean>("relink_recording", { id });
}

export function onLibraryChanged(handler: () => void) {
  return listen("library://changed", handler);
}
