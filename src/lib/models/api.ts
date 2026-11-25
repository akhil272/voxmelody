import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import type { LocationStatus, PackId, PackStatus, SystemInfo } from "./types.js";

export function listPacks() {
  return invoke<PackStatus[]>("list_packs");
}

export function downloadPack(id: PackId) {
  return invoke<void>("download_pack", { id });
}

export function pausePack(id: PackId) {
  return invoke<void>("pause_pack", { id });
}

export function removePack(id: PackId) {
  return invoke<void>("remove_pack", { id });
}

export function getSystemInfo() {
  return invoke<SystemInfo>("system_info");
}

export function getLocation() {
  return invoke<LocationStatus>("models_location");
}

export function changeLocation() {
  return invoke<void>("change_models_location");
}

export function restoreLocation() {
  return invoke<void>("restore_models_location");
}

export function onLocation(handler: (status: LocationStatus) => void) {
  return listen<LocationStatus>("models://location", (event) => handler(event.payload));
}

export function onPackStatus(handler: (status: PackStatus) => void) {
  return listen<PackStatus>("models://status", (event) => handler(event.payload));
}
