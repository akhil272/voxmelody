import type { RecordingKind } from "#lib/library/types.js";
import { COPY } from "./copy.js";
import { PACK_UI } from "./packs.js";

interface RecordingUi {
  title: string;
  tone: "voice" | "music";
  button: "default" | "secondary";
}

export const RECORDING_UI: Record<RecordingKind, RecordingUi> = {
  ...PACK_UI,
  mix: { title: COPY.mix.title, tone: "voice", button: "default" },
};
