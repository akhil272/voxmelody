import { FolderIcon, HomeIcon, InfoIcon, MicIcon, MusicIcon, SettingsIcon } from "lucide-react";
import { COPY } from "./copy";

export const ROUTES = {
  home: "/",
  voice: "/voice/",
  music: "/music/",
  library: "/library/",
  settings: "/settings/",
  about: "/about/",
} as const;

export const MAIN_NAV = [
  { href: ROUTES.home, label: COPY.nav.home, icon: HomeIcon },
  { href: ROUTES.voice, label: COPY.nav.voice, icon: MicIcon },
  { href: ROUTES.music, label: COPY.nav.music, icon: MusicIcon },
  { href: ROUTES.library, label: COPY.nav.library, icon: FolderIcon },
  { href: ROUTES.settings, label: COPY.nav.settings, icon: SettingsIcon },
];

export const FOOTER_NAV = [{ href: ROUTES.about, label: COPY.nav.about, icon: InfoIcon }];
