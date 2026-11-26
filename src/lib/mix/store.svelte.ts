import { DEFAULT_INTRO, DEFAULT_MUSIC_VOLUME, DEFAULT_OUTRO } from "#lib/constants.js";
import { exportRecording, loadRecording } from "#lib/library/api.js";
import type { Recording } from "#lib/library/types.js";
import type { Clip } from "#lib/takes/types.js";
import { toMixError } from "#lib/utils.js";
import * as api from "./api.js";
import type { MixError } from "./types.js";

class MixStore {
  voiceId = $state<number | null>(null);
  musicId = $state<number | null>(null);
  volume = $state(DEFAULT_MUSIC_VOLUME);
  intro = $state(DEFAULT_INTRO);
  outro = $state(DEFAULT_OUTRO);
  creating = $state(false);
  clip = $state<Clip | null>(null);
  error = $state<MixError | null>(null);
  exported = $state(false);

  async create(voice: Recording, music: Recording) {
    if (this.creating) return;
    this.creating = true;
    this.error = null;
    try {
      const take = await api.createMix({
        voice: voice.id,
        music: music.id,
        volume: this.volume / 100,
        intro: this.intro,
        outro: this.outro,
      });
      const clip = { ...take, title: voice.title, url: await loadRecording(take.id) };
      if (this.clip) URL.revokeObjectURL(this.clip.url);
      this.clip = clip;
      this.exported = false;
    } catch (error) {
      this.error = toMixError(error);
    } finally {
      this.creating = false;
    }
  }

  /** Drops a clip that was deleted from the library. */
  forget(id: number) {
    if (this.clip?.id !== id) return;
    URL.revokeObjectURL(this.clip.url);
    this.clip = null;
  }

  async export() {
    if (!this.clip) return;
    this.error = null;
    try {
      this.exported = await exportRecording(this.clip.id, this.clip.title);
    } catch {
      this.error = "output";
    }
  }
}

export const mix = new MixStore();
