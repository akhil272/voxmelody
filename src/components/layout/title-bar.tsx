"use client";

import { useSyncExternalStore } from "react";
import { MinusIcon, SquareIcon, XIcon } from "lucide-react";
import { Logo } from "@/components/brand/logo";
import { Button } from "@/components/ui/button";
import { COPY } from "@/lib/constants";
import { closeWindow, cn, isMacPlatform, minimizeWindow, subscribeNever, toggleMaximizeWindow } from "@/lib/utils";

const CONTROLS = [
  { label: COPY.window.minimize, icon: MinusIcon, action: minimizeWindow, variant: "ghost" },
  { label: COPY.window.maximize, icon: SquareIcon, action: toggleMaximizeWindow, variant: "ghost" },
  { label: COPY.window.close, icon: XIcon, action: closeWindow, variant: "close" },
] as const;

export function TitleBar() {
  const isMac = useSyncExternalStore(subscribeNever, isMacPlatform, () => false);

  return (
    <header
      data-tauri-drag-region
      className={cn("flex h-11 shrink-0 items-center justify-between border-b bg-card pl-4", isMac && "pl-20")}
    >
      <div className="pointer-events-none flex items-center gap-2 text-sm font-semibold">
        <Logo decorative className="h-4" />
        {COPY.appName}
      </div>
      {!isMac && (
        <div className="flex h-full">
          {CONTROLS.map(({ label, icon: Icon, action, variant }) => (
            <Button key={label} variant={variant} size="window" aria-label={label} onClick={action}>
              <Icon />
            </Button>
          ))}
        </div>
      )}
    </header>
  );
}
