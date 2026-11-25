import { invoke } from "@tauri-apps/api/core";
import { toAudioUrl } from "#lib/utils.js";
import type { CloneConsent, ClonedVoice, Sample } from "./types.js";

export function listClones() {
  return invoke<ClonedVoice[]>("list_clones");
}

export function checkSample(samples: Float32Array<ArrayBuffer>) {
  return invoke<Sample>("check_sample", samples.buffer);
}

export function discardSample() {
  return invoke<void>("discard_sample");
}

/** A playable address for the checked recording. Revoke it after use. */
export async function loadSample() {
  return toAudioUrl(await invoke<ArrayBuffer>("read_sample"));
}

export function createClone(name: string, consent: CloneConsent) {
  return invoke<ClonedVoice>("create_clone", { name, consent });
}

/** A playable address. Revoke it after use. */
export async function loadClone(id: string) {
  return toAudioUrl(await invoke<ArrayBuffer>("read_clone", { id }));
}

export function renameClone(id: string, name: string) {
  return invoke<void>("rename_clone", { id, name });
}

export function deleteClone(id: string) {
  return invoke<void>("delete_clone", { id });
}
