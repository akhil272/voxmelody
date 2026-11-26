import { openUrl } from "@tauri-apps/plugin-opener";

/** Opens a web link in the default browser instead of the app window. */
export function openExternal(url: string): void {
  openUrl(url).catch(() => undefined);
}
