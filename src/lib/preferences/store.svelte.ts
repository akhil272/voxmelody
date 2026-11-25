import { isTauri } from "@tauri-apps/api/core";
import type { ModelKind } from "#lib/models/types.js";
import * as api from "./api.js";
import type { Preferences } from "./types.js";

class PreferenceStore {
  #values = $state<Preferences>({ advanced: false, voiceModel: "", musicModel: "", favourites: [] });

  advanced = $derived(this.#values.advanced);

  async init() {
    if (isTauri()) this.#values = await api.readPreferences();
  }

  setAdvanced(advanced: boolean) {
    this.#change({ advanced });
  }

  model(kind: ModelKind) {
    return kind === "voice" ? this.#values.voiceModel : this.#values.musicModel;
  }

  setModel(kind: ModelKind, id: string) {
    this.#change(kind === "voice" ? { voiceModel: id } : { musicModel: id });
  }

  isFavourite(id: string) {
    return this.#values.favourites.includes(id);
  }

  toggleFavourite(id: string) {
    const { favourites } = this.#values;
    this.#change({ favourites: this.isFavourite(id) ? favourites.filter((item) => item !== id) : [...favourites, id] });
  }

  #change(values: Partial<Preferences>) {
    this.#values = { ...this.#values, ...values };
    // The choice still applies until the app closes when it cannot be saved.
    if (isTauri()) void api.savePreferences($state.snapshot(this.#values)).catch(() => undefined);
  }
}

export const preferences = new PreferenceStore();
