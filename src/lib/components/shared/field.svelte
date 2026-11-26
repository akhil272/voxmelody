<script lang="ts">
  import type { Snippet } from "svelte";

  let { label, hint, children }: { label: string; hint?: string; children: Snippet } = $props();
</script>

<div class="flex flex-col gap-1.5">
  <div class="flex items-center justify-between text-sm">
    <span class="font-medium">{label}</span>
    {#if hint}<span class="text-muted-foreground tabular-nums">{hint}</span>{/if}
  </div>
  {@render children()}
</div>
