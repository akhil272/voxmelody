import { isTauri } from "@tauri-apps/api/core";
import { DEFAULT_SPEED, MAX_SCRIPT_LENGTH } from "#lib/constants.js";
import { countCharacters, toFileName, toVoiceError } from "#lib/utils.js";
import * as api from "./api.js";
import type { Clip, Voice, VoiceError } from "./types.js";

type Activity = "idle" | "preview" | "generate";

class VoiceStore {
  voices = $state<Voice[]>([]);
  voiceId = $state("");
  script = $state("");
  speed = $state(DEFAULT_SPEED);
  activity = $state<Activity>("idle");
  progress = $state(0);
  clip = $state<Clip | null>(null);
  error = $state<VoiceError | null>(null);
  exported = $state(false);
  #preview: HTMLAudioElement | null = null;

  voice = $derived(this.voices.find((voice) => voice.id === this.voiceId));
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
    this.voiceId = this.voices[0]?.id ?? "";
  }

  setVoice(id: string) {
    this.voiceId = id;
  }

  clearError() {
    this.error = null;
  }

  setLanguage(language: string) {
    this.voiceId = this.voices.find((voice) => voice.language === language)?.id ?? this.voiceId;
  }

  async playPreview() {
    const clip = await this.#speak("preview");
    if (!clip) return;
    this.#preview = new Audio(clip.url);
    await this.#preview.play();
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

  async export() {
    if (!this.clip) return;
    this.error = null;
    try {
      this.exported = await api.exportTake(this.clip.id, `${toFileName(this.clip.title)}.wav`);
    } catch (error) {
      this.error = toVoiceError(error);
    }
  }

  #stopPreview() {
    if (!this.#preview) return;
    this.#preview.pause();
    URL.revokeObjectURL(this.#preview.src);
    this.#preview = null;
  }

  async #speak(activity: Exclude<Activity, "idle">): Promise<Clip | null> {
    if (!this.ready) return null;
    this.#stopPreview();
    this.activity = activity;
    this.progress = 0;
    this.error = null;
    const title = this.script.trim();
    try {
      const take = await api.speak({
        text: this.script,
        voice: this.voiceId,
        speed: this.speed,
        preview: activity === "preview",
      });
      const audio = new Blob([await api.readTake(take.id)], { type: "audio/wav" });
      return { ...take, title, url: URL.createObjectURL(audio) };
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
