import type { ModelFilter, ModelKind, ModelSize, ModelSort } from "#lib/models/types.js";
import { COPY } from "./copy.js";

export const ALL = "all";
const { kindFilter, sizes, sorts } = COPY.models;

export const DEFAULT_FILTER: ModelFilter = {
  search: "",
  kind: ALL,
  licence: ALL,
  size: "any",
  sort: "recommended",
  favourites: false,
};

export const MAX_MODEL_BYTES: Record<ModelSize, number> = {
  any: Infinity,
  small: 100e6,
  medium: 1e9,
  large: 4e9,
};

export const KIND_OPTIONS: { value: ModelKind | typeof ALL; label: string }[] = [
  { value: ALL, label: kindFilter.all },
  { value: "voice", label: kindFilter.voice },
  { value: "music", label: kindFilter.music },
];

export const SIZE_OPTIONS: { value: ModelSize; label: string }[] = [
  { value: "any", label: sizes.any },
  { value: "small", label: sizes.small },
  { value: "medium", label: sizes.medium },
  { value: "large", label: sizes.large },
];

export const SORT_OPTIONS: { value: ModelSort; label: string }[] = [
  { value: "recommended", label: sorts.recommended },
  { value: "newest", label: sorts.newest },
  { value: "smallest", label: sorts.smallest },
  { value: "name", label: sorts.name },
];
