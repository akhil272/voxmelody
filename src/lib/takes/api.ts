import { invoke } from "@tauri-apps/api/core";
import { toAudioUrl } from "#lib/utils.js";

/** A playable address for a voice preview. Revoke it after use. */
export async function loadTake(id: number) {
  return toAudioUrl(await invoke<ArrayBuffer>("read_take", { id }));
}
