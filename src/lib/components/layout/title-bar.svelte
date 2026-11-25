<script lang="ts">
  import MinusIcon from "@lucide/svelte/icons/minus";
  import SquareIcon from "@lucide/svelte/icons/square";
  import XIcon from "@lucide/svelte/icons/x";
  import Logo from "#lib/components/brand/logo.svelte";
  import { Button } from "#lib/components/ui/button/index.js";
  import { COPY } from "#lib/constants.js";
  import { closeWindow, cn, isMacPlatform, minimizeWindow, toggleMaximizeWindow } from "#lib/utils.js";

  const CONTROLS = [
    { label: COPY.window.minimize, icon: MinusIcon, action: minimizeWindow, variant: "ghost" },
    { label: COPY.window.maximize, icon: SquareIcon, action: toggleMaximizeWindow, variant: "ghost" },
    { label: COPY.window.close, icon: XIcon, action: closeWindow, variant: "close" },
  ] as const;

  const isMac = isMacPlatform();
</script>

<header
  data-tauri-drag-region
  class={cn("flex h-11 shrink-0 items-center justify-between border-b bg-card pl-4", isMac && "pl-20")}
>
  <div class="pointer-events-none flex items-center gap-2 text-sm font-semibold">
    <Logo class="h-4" />
    {COPY.appName}
  </div>
  {#if !isMac}
    <div class="flex h-full">
      {#each CONTROLS as { label, icon: Icon, action, variant } (label)}
        <Button {variant} size="window" aria-label={label} onclick={action}><Icon /></Button>
      {/each}
    </div>
  {/if}
</header>
