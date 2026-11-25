import type { LucideIcon } from "@lucide/svelte";
import MicIcon from "@lucide/svelte/icons/mic";
import MusicIcon from "@lucide/svelte/icons/music";
import type { PackId } from "#lib/models/types.js";
import { COPY } from "./copy.js";

interface PackUi {
  title: string;
  description: string;
  icon: LucideIcon;
  tone: "voice" | "music";
  button: "default" | "secondary";
}

export const PACK_UI: Record<PackId, PackUi> = {
  voice: { ...COPY.models.packs.voice, icon: MicIcon, tone: "voice", button: "default" },
  music: { ...COPY.models.packs.music, icon: MusicIcon, tone: "music", button: "secondary" },
};

export const PACK_IDS: PackId[] = ["voice", "music"];
