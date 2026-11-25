<script lang="ts">
  import { cn, waveformHeights } from "#lib/utils.js";

  let { tone, bars = 25 }: { tone: "voice" | "music"; bars?: number } = $props();
</script>

<div class="flex h-14 shrink-0 items-center gap-[3px]" aria-hidden="true">
  {#each waveformHeights(bars) as height, index (index)}
    <span
      class={cn("w-[2px] rounded-full", tone === "voice" ? "bg-teal-bright" : "bg-secondary")}
      style:height="{height * 100}%"
    ></span>
  {/each}
</div>
