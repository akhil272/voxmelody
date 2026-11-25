import { isTauri } from "@tauri-apps/api/core";
import { AUTO, DEFAULT_PAUSE, DEFAULT_SPEED, MAX_SCRIPT_LENGTH } from "#lib/constants.js";
import { exportRecording, loadRecording } from "#lib/library/api.js";
import { models } from "#lib/models/store.svelte.js";
import { preferences } from "#lib/preferences/store.svelte.js";
import { loadTake } from "#lib/takes/api.js";
import type { Clip } from "#lib/takes/types.js";
import { closestVoice, countCharacters, toVoiceError } from "#lib/utils.js";
import * as api from "./api.js";
import type { Voice, VoiceError } from "./types.js";

type Activity = "idle" | "preview" | "generate";

class VoiceStore {
  voices = $state<Voice[]>([]);
  choice = $state<Voice | null>(null);
  script = $state("");
  speed = $state(DEFAULT_SPEED);
  pause = $state(DEFAULT_PAUSE);
  threads = $state(AUTO);
  steps = $state(AUTO);
  activity = $state<Activity>("idle");
  progress = $state(0);
  clip = $state<Clip | null>(null);
  error = $state<VoiceError | null>(null);
  exported = $state(false);
  #preview: HTMLAudioElement | null = null;

  available = $derived(this.voices.filter(({ model }) => model === models.selected("voice")?.id));
  voice = $derived(closestVoice(this.available, this.choice));
  length = $derived(countCharacters(this.script));
  ready = $derived(
    this.activity === "idle" && !!this.voice && this.script.trim() !== "" && this.length <= MAX_SCRIPT_LENGTH,
  );

  async init() {
    if (!isTauri()) return;
    await api.onProgress((value) => {
      if (this.activity !== "idle") this.progress = value;
    });
    this.voices = await api.listVoices();
  }

  setVoice(id: string) {
    this.choice = this.available.find((voice) => voice.id === id) ?? null;
  }

  clearError() {
    this.error = null;
  }

  setLanguage(language: string) {
    const speaking = this.available.filter((voice) => voice.language === language);
    this.choice = speaking.find(({ name }) => name === this.voice?.name) ?? speaking[0] ?? null;
  }

  async playPreview() {
    const clip = await this.#speak("preview");
    if (!clip) return;
    this.#preview = new Audio(clip.url);
    // Rejects when another request stops the preview before it starts.
    await this.#preview.play().catch(() => undefined);
  }

  async generate() {
    const clip = await this.#speak("generate");
    if (!clip) return;
    if (this.clip) URL.revokeObjectURL(this.clip.url);
    this.clip = clip;
    this.exported = false;
  }

  async cancel() {
    await api.cancelSpeech();
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

  #stopPreview() {
    if (!this.#preview) return;
    this.#preview.pause();
    URL.revokeObjectURL(this.#preview.src);
    this.#preview = null;
  }

  async #speak(activity: Exclude<Activity, "idle">): Promise<Clip | null> {
    const { voice } = this;
    if (!this.ready || !voice) return null;
    const { advanced } = preferences;
    this.#stopPreview();
    this.activity = activity;
    this.progress = 0;
    this.error = null;
    const title = this.script.trim();
    try {
      const take = await api.speak({
        text: this.script,
        model: voice.model,
        voice: voice.id,
        speed: this.speed,
        preview: activity === "preview",
        // Hidden controls never change the result.
        pause: advanced ? this.pause : DEFAULT_PAUSE,
        threads: advanced ? this.threads : AUTO,
        steps: advanced ? this.steps : AUTO,
      });
      const url = await (activity === "preview" ? loadTake(take.id) : loadRecording(take.id));
      return { ...take, title, url };
    } catch (error) {
      const code = toVoiceError(error);
      this.error = code === "cancelled" ? null : code;
      return null;
    } finally {
      this.activity = "idle";
    }
  }
}

export const voice = new VoiceStore();
