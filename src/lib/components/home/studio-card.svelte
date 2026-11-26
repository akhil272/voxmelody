<script lang="ts">
  import type { LucideIcon } from "@lucide/svelte";
  import type { Snippet } from "svelte";
  import Waveform from "#lib/components/brand/waveform.svelte";
  import { Card } from "#lib/components/ui/card/index.js";
  import { cn } from "#lib/utils.js";

  interface Props {
    title: string;
    description: string;
    icon: LucideIcon;
    tone: "voice" | "music";
    children: Snippet;
  }

  let { title, description, icon: Icon, tone, children }: Props = $props();
</script>

<Card>
  <div class="flex items-center justify-between gap-4">
    <div>
      <h2 class="flex items-center gap-3 font-semibold">
        <Icon class={cn("size-5", tone === "voice" ? "text-primary" : "text-secondary")} />
        {title}
      </h2>
      <p class="mt-1 text-sm text-muted-foreground">{description}</p>
    </div>
    <Waveform {tone} />
  </div>
  {@render children()}
</Card>
