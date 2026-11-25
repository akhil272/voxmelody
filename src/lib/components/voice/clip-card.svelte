<script lang="ts">
  import DownloadIcon from "@lucide/svelte/icons/download";
  import AudioPlayer from "#lib/components/shared/audio-player.svelte";
  import { Badge } from "#lib/components/ui/badge/index.js";
  import { Button } from "#lib/components/ui/button/index.js";
  import { Card } from "#lib/components/ui/card/index.js";
  import { COPY } from "#lib/constants.js";
  import { voice } from "#lib/voice/store.svelte.js";

  const copy = COPY.voice;
</script>

{#if voice.clip}
  <Card class="shrink-0">
    <div class="flex items-center justify-between gap-4">
      <h2 class="truncate font-semibold">{voice.clip.title}</h2>
      <div class="flex shrink-0 items-center gap-3">
        {#if voice.exported}<Badge tone="success">{copy.exported}</Badge>{/if}
        <Button variant="outline" onclick={() => voice.export()}>
          <DownloadIcon />
          {copy.export}
        </Button>
      </div>
    </div>
    {#key voice.clip.url}
      <AudioPlayer url={voice.clip.url} peaks={voice.clip.peaks} durationMs={voice.clip.durationMs} />
    {/key}
  </Card>
{/if}
