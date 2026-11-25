import { invoke } from "@tauri-apps/api/core";
import type { Clip, Take } from "./types.js";

export async function loadClip(take: Take, title: string): Promise<Clip> {
  const audio = new Blob([await invoke<ArrayBuffer>("read_take", { id: take.id })], { type: "audio/wav" });
  return { ...take, title, url: URL.createObjectURL(audio) };
}

/** Resolves to false when the user closes the save dialog. */
export function exportTake(id: number, name: string) {
  return invoke<boolean>("export_take", { id, name });
}
