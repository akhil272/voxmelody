<script lang="ts">
  import PauseIcon from "@lucide/svelte/icons/pause";
  import PlayIcon from "@lucide/svelte/icons/play";
  import { Button } from "#lib/components/ui/button/index.js";
  import { COPY } from "#lib/constants.js";
  import { cn, formatTime } from "#lib/utils.js";

  interface Props {
    url: string;
    peaks: number[];
    durationMs: number;
    tone?: "voice" | "music";
  }

  let { url, peaks, durationMs, tone = "voice" }: Props = $props();

  let paused = $state(true);
  let currentTime = $state(0);

  const copy = COPY.player;
  const duration = $derived(durationMs / 1000);
  const loudest = $derived(Math.max(...peaks, 0.01));
</script>

<audio src={url} bind:paused bind:currentTime></audio>

<div class="flex items-center gap-4">
  <Button
    variant={tone === "voice" ? "default" : "secondary"}
    size="round"
    aria-label={paused ? copy.play : copy.pause}
    onclick={() => (paused = !paused)}
  >
    {#if paused}<PlayIcon />{:else}<PauseIcon />{/if}
  </Button>
  <div class="relative h-12 min-w-0 flex-1 rounded-md focus-within:ring-3 focus-within:ring-ring/40">
    <div class="flex h-full items-center gap-px" aria-hidden="true">
      {#each peaks as peak, index (index)}
        <span
          class={cn(
            "min-h-0.5 flex-1 rounded-full",
            index / peaks.length >= currentTime / duration
              ? "bg-border"
              : tone === "voice"
                ? "bg-primary"
                : "bg-secondary",
          )}
          style:height="{(peak / loudest) * 100}%"
        ></span>
      {/each}
    </div>
    <input
      type="range"
      class="absolute inset-0 size-full cursor-pointer opacity-0"
      min="0"
      max={duration}
      step="0.01"
      aria-label={copy.seek}
      aria-valuetext={formatTime(currentTime)}
      bind:value={currentTime}
    />
  </div>
  <span class="shrink-0 text-sm text-muted-foreground tabular-nums">
    {formatTime(currentTime)} / {formatTime(duration)}
  </span>
</div>
