import type { CloneConsent } from "#lib/clones/types.js";
import { COPY } from "./copy.js";

export const SAMPLE_RATE = 24_000;
export const MAX_SAMPLE_BYTES = 50e6;
export const MAX_RECORDING_SECONDS = 30;
export const MAX_VOICE_NAME_LENGTH = 40;

export const CONSENT_OPTIONS: { value: CloneConsent; label: string }[] = [
  { value: "own", label: COPY.clones.owners.own },
  { value: "permission", label: COPY.clones.owners.permission },
];
