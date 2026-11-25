import { COPY } from "#lib/constants.js";
import type { Recording } from "#lib/library/types.js";
import { formatTime } from "./time.js";

/** The chosen recording, or the newest when it is gone. */
export function findRecording(recordings: Recording[], id: number | null): Recording | undefined {
  return recordings.find((recording) => recording.id === id) ?? recordings[0];
}

export function toRecordingOptions(recordings: Recording[]) {
  return recordings.map(({ id, title, durationMs }) => ({
    value: String(id),
    label: COPY.mix.option(title, formatTime(durationMs / 1000)),
  }));
}
