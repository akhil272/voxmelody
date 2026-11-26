import { isTauri } from "@tauri-apps/api/core";
import { mix } from "#lib/mix/store.svelte.js";
import { music } from "#lib/music/store.svelte.js";
import { toLibraryError } from "#lib/utils.js";
import { voice } from "#lib/voice/store.svelte.js";
import * as api from "./api.js";
import type { LibraryError, Recording, SaveFolder } from "./types.js";

/** Something that happened to one recording. */
interface Notice<T> {
  id: number;
  value: T;
}

class LibraryStore {
  recordings = $state<Recording[]>([]);
  loaded = $state(false);
  playing = $state<Notice<string> | null>(null);
  error = $state<Notice<LibraryError> | null>(null);
  exportedId = $state<number | null>(null);
  loadError = $state<LibraryError | null>(null);
  saveFolder = $state<SaveFolder | null>(null);
  saveFolderError = $state<LibraryError | null>(null);

  voices = $derived(this.recordings.filter(({ kind, available }) => kind === "voice" && available));
  tracks = $derived(this.recordings.filter(({ kind, available }) => kind === "music" && available));

  async init() {
    if (!isTauri()) return;
    await api.onLibraryChanged(() => void this.refresh());
    await this.refresh();
  }

  async refresh() {
    if (!isTauri()) return;
    try {
      this.recordings = await api.listRecordings();
      this.loadError = null;
    } catch (error) {
      this.loadError = toLibraryError(error);
    }
    this.loaded = true;
  }

  async play(id: number) {
    this.stop();
    await this.#run(id, async () => {
      this.playing = { id, value: await api.loadRecording(id) };
    });
  }

  async loadSaveFolder() {
    if (!isTauri()) return;
    await this.#runFolder(async () => {
      this.saveFolder = await api.readSaveFolder();
    });
  }

  changeSaveFolder() {
    return this.#runFolder(async () => {
      if (await api.changeSaveFolder()) this.saveFolder = await api.readSaveFolder();
    });
  }

  restoreSaveFolder() {
    return this.#runFolder(async () => {
      await api.restoreSaveFolder();
      this.saveFolder = await api.readSaveFolder();
    });
  }

  async #runFolder(action: () => Promise<unknown>) {
    this.saveFolderError = null;
    try {
      await action();
    } catch (error) {
      this.saveFolderError = toLibraryError(error);
    }
  }

  stop() {
    if (this.playing) URL.revokeObjectURL(this.playing.value);
    this.playing = null;
  }

  rename(id: number, title: string) {
    return this.#run(id, () => api.renameRecording(id, title));
  }

  async export({ id, title }: Recording) {
    this.exportedId = null;
    await this.#run(id, async () => {
      if (await api.exportRecording(id, title)) this.exportedId = id;
    });
  }

  relink(id: number) {
    return this.#run(id, () => api.relinkRecording(id));
  }

  async delete(id: number) {
    if (this.playing?.id === id) this.stop();
    await this.#run(id, async () => {
      await api.deleteRecording(id);
      voice.forget(id);
      music.forget(id);
      mix.forget(id);
    });
  }

  async #run(id: number, action: () => Promise<unknown>) {
    this.error = null;
    try {
      await action();
    } catch (error) {
      this.error = { id, value: toLibraryError(error) };
    }
    await this.refresh();
  }
}

export const library = new LibraryStore();
