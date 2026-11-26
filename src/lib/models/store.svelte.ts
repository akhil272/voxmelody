import { isTauri } from "@tauri-apps/api/core";
import { preferences } from "#lib/preferences/store.svelte.js";
import { toModelError } from "#lib/utils.js";
import * as api from "./api.js";
import type { LocationStatus, ModelError, ModelInfo, ModelKind, PackStatus, SystemInfo } from "./types.js";

class ModelStore {
  catalog = $state<ModelInfo[]>([]);
  packs = $state<Partial<Record<string, PackStatus>>>({});
  system = $state<SystemInfo | null>(null);
  graphics = $state<string | null>(null);
  location = $state<LocationStatus | null>(null);
  locationError = $state<ModelError | null>(null);

  /** The model that speaks in cloned voices. */
  cloning = $derived(this.catalog.find((model) => model.voice?.family === "pocket"));

  async init() {
    if (!isTauri()) return;
    // Subscribe first so no update is missed between listing and listening.
    await api.onPackStatus((status) => this.#apply(status));
    await api.onLocation((status) => (this.location = status));
    this.location = await api.getLocation();
    this.catalog = await api.listModels();
    await this.#refreshPacks();
    await this.refreshSystem();
  }

  find(id: string) {
    return this.catalog.find((model) => model.id === id);
  }

  isInstalled(id: string) {
    return this.packs[id]?.state === "installed";
  }

  installed(kind: ModelKind) {
    return this.catalog.filter((model) => model.kind === kind && this.isInstalled(model.id));
  }

  /** The first model of a kind is the recommended one. */
  recommended(kind: ModelKind) {
    return this.catalog.find((model) => model.kind === kind);
  }

  /** The model that generates, if any is installed. */
  selected(kind: ModelKind) {
    const installed = this.installed(kind);
    return installed.find((model) => model.id === preferences.model(kind)) ?? installed[0];
  }

  async #refreshSizes() {
    for (const { id, downloadBytes } of await api.listPacks()) {
      const pack = this.packs[id];
      if (pack) pack.downloadBytes = downloadBytes;
    }
  }

  async #refreshPacks() {
    for (const status of await api.listPacks()) this.packs[status.id] = status;
  }

  async refreshSystem() {
    this.system = await api.getSystemInfo();
  }

  async loadGraphics() {
    if (isTauri()) this.graphics = await api.getGraphicsName();
  }

  download(id: string) {
    return this.#run(id, api.downloadPack);
  }

  pause(id: string) {
    return this.#run(id, api.pausePack);
  }

  remove(id: string) {
    return this.#run(id, api.removePack);
  }

  changeLocation() {
    return this.#relocate(api.changeLocation);
  }

  restoreLocation() {
    return this.#relocate(api.restoreLocation);
  }

  async #relocate(action: () => Promise<void>) {
    this.locationError = null;
    try {
      await action();
      await this.refreshSystem();
    } catch (error) {
      this.locationError = toModelError(error);
    }
  }

  #apply(status: PackStatus) {
    const finished = this.packs[status.id]?.state !== status.state;
    this.packs[status.id] = status;
    if (!finished || !this.system) return;
    void this.refreshSystem();
    // Sizes left to download depend on what is installed.
    if (status.state === "installed" || status.state === "available") void this.#refreshSizes();
  }

  async #run(id: string, action: (id: string) => Promise<void>) {
    this.#setError(id, null);
    try {
      await action(id);
    } catch (error) {
      this.#setError(id, toModelError(error));
    }
  }

  #setError(id: string, error: ModelError | null) {
    const pack = this.packs[id];
    if (pack) pack.error = error;
  }
}

export const models = new ModelStore();
