import { isTauri } from "@tauri-apps/api/core";
import {
  AUTO,
  AUTO_BPM,
  AUTO_KEY,
  DEFAULT_BPM,
  DEFAULT_CREATIVITY,
  DEFAULT_GUIDANCE,
  DEFAULT_LENGTH,
  MAX_PROMPT_LENGTH,
} from "#lib/constants.js";
import { exportRecording, loadRecording } from "#lib/library/api.js";
import { models } from "#lib/models/store.svelte.js";
import { preferences } from "#lib/preferences/store.svelte.js";
import type { Clip } from "#lib/takes/types.js";
import { countCharacters, isSeed, parseSeed, toMusicError } from "#lib/utils.js";
import * as api from "./api.js";
import type { MusicError, MusicOptions, Processor, TempoMode } from "./types.js";

class MusicStore {
  prompt = $state("");
  seconds = $state(DEFAULT_LENGTH);
  tempo = $state<TempoMode>("auto");
  bpm = $state(DEFAULT_BPM);
  key = $state(AUTO_KEY);
  seed = $state("");
  /** Models need very different numbers of steps. */
  #customSteps = $state<Record<string, number>>({});
  guidance = $state(DEFAULT_GUIDANCE);
  creativity = $state(DEFAULT_CREATIVITY);
  processor = $state<Processor>("auto");
  generating = $state(false);
  progress = $state(0);
  clip = $state<Clip | null>(null);
  error = $state<MusicError | null>(null);
  exported = $state(false);

  length = $derived(countCharacters(this.prompt.trim()));
  model = $derived(models.selected("music"));
  steps = $derived(this.#customSteps[this.model?.id ?? ""] ?? this.model?.music?.steps ?? AUTO);
  ready = $derived(
    !this.generating &&
      this.length > 0 &&
      this.length <= MAX_PROMPT_LENGTH &&
      (!preferences.advanced || isSeed(this.seed)),
  );

  async init() {
    if (!isTauri()) return;
    await api.onProgress((value) => {
      if (this.generating) this.progress = value;
    });
  }

  clearError() {
    this.error = null;
  }

  setSteps(steps: number) {
    if (this.model) this.#customSteps[this.model.id] = steps;
  }

  /** Hidden controls never change the result. */
  #options(): MusicOptions {
    const { advanced } = preferences;
    return {
      seed: advanced ? parseSeed(this.seed) : null,
      steps: advanced ? this.steps : AUTO,
      guidance: advanced && this.model?.music?.guidance ? this.guidance : AUTO,
      creativity: advanced ? this.creativity : AUTO,
      cpuOnly: advanced && this.processor === "cpu",
    };
  }

  async generate() {
    const { model } = this;
    if (!this.ready || !model) return;
    this.generating = true;
    this.progress = 0;
    this.error = null;
    const prompt = this.prompt.trim();
    try {
      const take = await api.compose({
        model: model.id,
        prompt,
        seconds: this.seconds,
        bpm: this.tempo === "custom" ? this.bpm : AUTO_BPM,
        key: this.key === AUTO_KEY ? "" : this.key,
        options: this.#options(),
      });
      const clip = { ...take, title: prompt, url: await loadRecording(take.id) };
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

export const music = new MusicStore();
