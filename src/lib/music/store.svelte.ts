import { isTauri } from "@tauri-apps/api/core";
import { AUTO_BPM, AUTO_KEY, DEFAULT_BPM, DEFAULT_LENGTH, MAX_PROMPT_LENGTH } from "#lib/constants.js";
import { exportTake, loadClip } from "#lib/takes/api.js";
import type { Clip } from "#lib/takes/types.js";
import { countCharacters, toFileName, toMusicError } from "#lib/utils.js";
import * as api from "./api.js";
import type { MusicError, TempoMode } from "./types.js";

class MusicStore {
  prompt = $state("");
  seconds = $state(DEFAULT_LENGTH);
  tempo = $state<TempoMode>("auto");
  bpm = $state(DEFAULT_BPM);
  key = $state(AUTO_KEY);
  generating = $state(false);
  progress = $state(0);
  clip = $state<Clip | null>(null);
  error = $state<MusicError | null>(null);
  exported = $state(false);

  length = $derived(countCharacters(this.prompt.trim()));
  ready = $derived(!this.generating && this.length > 0 && this.length <= MAX_PROMPT_LENGTH);

  async init() {
    if (!isTauri()) return;
    await api.onProgress((value) => {
      if (this.generating) this.progress = value;
    });
  }

  clearError() {
    this.error = null;
  }

  async generate() {
    if (!this.ready) return;
    this.generating = true;
    this.progress = 0;
    this.error = null;
    const prompt = this.prompt.trim();
    try {
      const take = await api.compose({
        prompt,
        seconds: this.seconds,
        bpm: this.tempo === "custom" ? this.bpm : AUTO_BPM,
        key: this.key === AUTO_KEY ? "" : this.key,
      });
      const clip = await loadClip(take, prompt);
      if (this.clip) URL.revokeObjectURL(this.clip.url);
      this.clip = clip;
      this.exported = false;
    } catch (error) {
      const code = toMusicError(error);
      this.error = code === "cancelled" ? null : code;
    } finally {
      this.generating = false;
    }
  }

  async cancel() {
    await api.cancelMusic();
  }

  async export() {
    if (!this.clip) return;
    this.error = null;
    try {
      this.exported = await exportTake(this.clip.id, `${toFileName(this.clip.title)}.wav`);
    } catch (error) {
      this.error = toMusicError(error);
    }
  }
}

export const music = new MusicStore();
