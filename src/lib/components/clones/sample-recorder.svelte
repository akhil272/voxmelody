<script lang="ts">
  import MicIcon from "@lucide/svelte/icons/mic";
  import SquareIcon from "@lucide/svelte/icons/square";
  import UploadIcon from "@lucide/svelte/icons/upload";
  import AudioPlayer from "#lib/components/shared/audio-player.svelte";
  import { Button } from "#lib/components/ui/button/index.js";
  import { clones } from "#lib/clones/store.svelte.js";
  import type { CloneConsent } from "#lib/clones/types.js";
  import { COPY, MAX_RECORDING_SECONDS } from "#lib/constants.js";
  import { formatTime } from "#lib/utils.js";

  let { owner }: { owner: CloneConsent } = $props();

  let picker: HTMLInputElement;

  const copy = COPY.clones;

  async function choose() {
    const file = picker.files?.[0];
    if (file) await clones.choose(file);
    // Choosing the same file again must also count.
    picker.value = "";
  }
</script>

<div class="flex flex-col gap-2 rounded-lg bg-muted p-4">
  <p class="text-sm font-medium">{copy.read}</p>
  <blockquote class="select-text">{copy.statements[owner]}</blockquote>
</div>
<input bind:this={picker} type="file" accept="audio/*" class="hidden" aria-label={copy.choose} onchange={choose} />
<div class="flex items-center gap-2">
  {#if clones.recording}
    <Button onclick={() => clones.stop()}>
      <SquareIcon />
      {copy.stop}
    </Button>
    <span class="ml-2 text-sm text-muted-foreground tabular-nums">
      {formatTime(clones.elapsed)} / {formatTime(MAX_RECORDING_SECONDS)}
    </span>
  {:else}
    <Button variant={clones.sample ? "outline" : "default"} disabled={clones.busy} onclick={() => clones.record()}>
      <MicIcon />
      {clones.sample ? copy.recordAgain : copy.record}
    </Button>
    <Button variant="outline" disabled={clones.busy} onclick={() => picker.click()}>
      <UploadIcon />
      {copy.choose}
    </Button>
  {/if}
</div>
{#if clones.sample}
  {#key clones.sample.url}
    <AudioPlayer url={clones.sample.url} peaks={clones.sample.peaks} durationMs={clones.sample.durationMs} />
  {/key}
{/if}
