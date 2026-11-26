import type { LibraryFilter, RecordingKind } from "#lib/library/types.js";
import { COPY } from "./copy.js";

export const MAX_TITLE_LENGTH = 80;

export const DEFAULT_LIBRARY_FILTER: LibraryFilter = "all";

export const LIBRARY_FILTERS: { value: LibraryFilter; label: string }[] = [
  { value: "all", label: COPY.library.filters.all },
  { value: "voice", label: COPY.library.filters.voice },
  { value: "music", label: COPY.library.filters.music },
  { value: "mix", label: COPY.library.filters.mix },
];

/** Songs are music with singing, so they sit with music. */
export const FILTER_KINDS: Record<Exclude<LibraryFilter, "all">, RecordingKind[]> = {
  voice: ["voice"],
  music: ["music", "song"],
  mix: ["mix"],
};
