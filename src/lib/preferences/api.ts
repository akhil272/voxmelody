import { invoke } from "@tauri-apps/api/core";
import type { Preferences } from "./types.js";

export function readPreferences() {
  return invoke<Preferences>("read_preferences");
}

export function savePreferences(preferences: Preferences) {
  return invoke<void>("save_preferences", { preferences });
}
