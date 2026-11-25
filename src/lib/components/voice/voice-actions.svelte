<script lang="ts">
  import PlayIcon from "@lucide/svelte/icons/play";
  import { Button } from "#lib/components/ui/button/index.js";
  import { Progress } from "#lib/components/ui/progress/index.js";
  import { COPY } from "#lib/constants.js";
  import { toPercent } from "#lib/utils.js";
  import { voice } from "#lib/voice/store.svelte.js";

  const copy = COPY.voice;
</script>

<div class="flex flex-col gap-3">
  {#if voice.activity === "idle"}
    <Button variant="outline" disabled={!voice.ready} onclick={() => voice.playPreview()}>
      <PlayIcon />
      {copy.preview}
    </Button>
    <Button disabled={!voice.ready} onclick={() => voice.generate()}>{copy.generate}</Button>
  {:else}
    <Progress value={toPercent(voice.progress, 1)} label={copy.generating} />
    <p class="text-sm text-muted-foreground" aria-live="polite">
      {voice.activity === "preview" ? copy.previewing : copy.generating}
    </p>
    <Button variant="outline" onclick={() => voice.cancel()}>{copy.cancel}</Button>
  {/if}
  {#if voice.error}
    <p role="alert" class="text-sm text-destructive">{copy.errors[voice.error]}</p>
  {/if}
</div>
