import { isTauri } from "@tauri-apps/api/core";
import { toModelError } from "#lib/utils.js";
import * as api from "./api.js";
import type { LocationStatus, ModelError, PackId, PackStatus, SystemInfo } from "./types.js";

class ModelStore {
  packs = $state<Partial<Record<PackId, PackStatus>>>({});
  system = $state<SystemInfo | null>(null);
  location = $state<LocationStatus | null>(null);
  locationError = $state<ModelError | null>(null);

  async init() {
    if (!isTauri()) return;
    // Subscribe first so no update is missed between listing and listening.
    await api.onPackStatus((status) => this.#apply(status));
    await api.onLocation((status) => (this.location = status));
    this.location = await api.getLocation();
    for (const status of await api.listPacks()) this.#apply(status);
    await this.refreshSystem();
  }

  async refreshSystem() {
    this.system = await api.getSystemInfo();
  }

  download(id: PackId) {
    return this.#run(id, api.downloadPack);
  }

  pause(id: PackId) {
    return this.#run(id, api.pausePack);
  }

  remove(id: PackId) {
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
    if (finished && this.system) void this.refreshSystem();
  }

  async #run(id: PackId, action: (id: PackId) => Promise<void>) {
    this.#setError(id, null);
    try {
      await action(id);
    } catch (error) {
      this.#setError(id, toModelError(error));
    }
  }

  #setError(id: PackId, error: ModelError | null) {
    const pack = this.packs[id];
    if (pack) pack.error = error;
  }
}

export const models = new ModelStore();
