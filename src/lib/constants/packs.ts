import type { LucideIcon } from "@lucide/svelte";
import MicIcon from "@lucide/svelte/icons/mic";
import MusicIcon from "@lucide/svelte/icons/music";
import type { ModelKind } from "#lib/models/types.js";
import { COPY } from "./copy.js";

interface PackUi {
  title: string;
  icon: LucideIcon;
  tone: "voice" | "music";
  button: "default" | "secondary";
}

export const PACK_UI: Record<ModelKind, PackUi> = {
  voice: { title: COPY.models.kinds.voice, icon: MicIcon, tone: "voice", button: "default" },
  music: { title: COPY.models.kinds.music, icon: MusicIcon, tone: "music", button: "secondary" },
};

export const MODEL_KINDS: ModelKind[] = ["voice", "music"];
