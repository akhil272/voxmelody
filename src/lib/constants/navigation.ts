import type { LucideIcon } from "@lucide/svelte";
import AudioLinesIcon from "@lucide/svelte/icons/audio-lines";
import BlendIcon from "@lucide/svelte/icons/blend";
import BoxIcon from "@lucide/svelte/icons/box";
import FolderIcon from "@lucide/svelte/icons/folder";
import HouseIcon from "@lucide/svelte/icons/house";
import InfoIcon from "@lucide/svelte/icons/info";
import MicIcon from "@lucide/svelte/icons/mic";
import MusicIcon from "@lucide/svelte/icons/music";
import SettingsIcon from "@lucide/svelte/icons/settings";
import { COPY } from "./copy.js";

export interface NavItem {
  href: string;
  label: string;
  icon: LucideIcon;
}

export const ROUTES = {
  home: "/",
  voice: "/voice",
  voices: "/voices",
  music: "/music",
  mix: "/mix",
  library: "/library",
  models: "/models",
  settings: "/settings",
  about: "/about",
} as const;

export const MAIN_NAV: NavItem[] = [
  { href: ROUTES.home, label: COPY.nav.home, icon: HouseIcon },
  { href: ROUTES.voice, label: COPY.nav.voice, icon: MicIcon },
  { href: ROUTES.voices, label: COPY.nav.voices, icon: AudioLinesIcon },
  { href: ROUTES.music, label: COPY.nav.music, icon: MusicIcon },
  { href: ROUTES.mix, label: COPY.nav.mix, icon: BlendIcon },
  { href: ROUTES.library, label: COPY.nav.library, icon: FolderIcon },
  { href: ROUTES.models, label: COPY.nav.models, icon: BoxIcon },
  { href: ROUTES.settings, label: COPY.nav.settings, icon: SettingsIcon },
];

export const FOOTER_NAV: NavItem[] = [{ href: ROUTES.about, label: COPY.nav.about, icon: InfoIcon }];
