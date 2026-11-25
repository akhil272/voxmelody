import { isTauri } from "@tauri-apps/api/core";
import { decodeSample, Recorder, toCloneError } from "#lib/utils.js";
import { voice } from "#lib/voice/store.svelte.js";
import * as api from "./api.js";
import type { CloneConsent, ClonedVoice, CloneError, Sample } from "./types.js";

class CloneStore {
  voices = $state<ClonedVoice[]>([]);
  loaded = $state(false);
  playing = $state<{ id: string; url: string } | null>(null);
  sample = $state<(Sample & { url: string }) | null>(null);
  recording = $state(false);
  /** Seconds recorded so far. */
  elapsed = $state(0);
  busy = $state(false);
  error = $state<CloneError | null>(null);
  #recorder = new Recorder((seconds) => (this.elapsed = seconds));

  async refresh() {
    if (!isTauri()) return;
    await this.#run(async () => {
      this.voices = await api.listClones();
    });
    this.loaded = true;
  }

  record() {
    return this.#run(async () => {
      await this.discard();
      this.elapsed = 0;
      this.recording = await this.#recorder.start((audio) => void this.#finish(audio));
    });
  }

  stop() {
    this.#recorder.stop();
  }

  choose(file: File) {
    return this.#run(async () => {
      await this.discard();
      await this.#check(file);
    });
  }

  /** Drops the recording and any that is still being made. */
  async discard() {
    this.#recorder.cancel();
    this.recording = false;
    if (this.sample) URL.revokeObjectURL(this.sample.url);
    this.sample = null;
    if (isTauri()) await api.discardSample();
  }

  /** Resolves to true when the voice was saved. */
  create(name: string, consent: CloneConsent) {
    return this.#run(async () => {
      await api.createClone(name, consent);
      await this.discard();
      await this.#reload();
    });
  }

  async play(id: string) {
    this.close();
    await this.#run(async () => {
      this.playing = { id, url: await api.loadClone(id) };
    });
  }

  close() {
    if (this.playing) URL.revokeObjectURL(this.playing.url);
    this.playing = null;
  }

  rename(id: string, name: string) {
    return this.#run(async () => {
      await api.renameClone(id, name);
      await this.#reload();
    });
  }

  delete(id: string) {
    if (this.playing?.id === id) this.close();
    return this.#run(async () => {
      await api.deleteClone(id);
      await this.#reload();
    });
  }

  async #reload() {
    this.voices = await api.listClones();
    await voice.refresh();
  }

  async #finish(audio: Blob | null) {
    this.recording = false;
    if (audio) await this.#run(() => this.#check(audio));
  }

  async #check(audio: Blob) {
    const sample = await api.checkSample(await decodeSample(audio));
    this.sample = { ...sample, url: await api.loadSample() };
  }

  async #run(action: () => Promise<void>) {
    this.error = null;
    this.busy = true;
    try {
      await action();
      return true;
    } catch (error) {
      this.error = toCloneError(error);
      return false;
    } finally {
      this.busy = false;
    }
  }
}

export const clones = new CloneStore();
