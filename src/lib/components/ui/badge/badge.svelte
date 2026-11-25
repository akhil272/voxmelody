<script lang="ts">
  import type { Snippet } from "svelte";
  import { cn } from "#lib/utils.js";

  let { tone = "neutral", children }: { tone?: "neutral" | "success" | "warning"; children: Snippet } = $props();
</script>

<span
  data-slot="badge"
  class={cn(
    "inline-flex h-6 shrink-0 items-center rounded-full px-2.5 text-xs font-medium",
    tone === "neutral" && "bg-muted text-muted-foreground",
    tone === "success" && "bg-accent text-accent-foreground",
    tone === "warning" && "bg-warning-muted text-warning",
  )}
>
  {@render children()}
</span>
