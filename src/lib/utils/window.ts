import { isTauri } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";

export async function minimizeWindow() {
  if (isTauri()) await getCurrentWindow().minimize();
}

export async function toggleMaximizeWindow() {
  if (isTauri()) await getCurrentWindow().toggleMaximize();
}

export async function closeWindow() {
  if (isTauri()) await getCurrentWindow().close();
}
