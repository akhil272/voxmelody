import { invoke } from "@tauri-apps/api/core";

export function getAssistantCommand() {
  return invoke<string | null>("assistant_command");
}
