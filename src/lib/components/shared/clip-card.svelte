<script lang="ts">
  import DownloadIcon from "@lucide/svelte/icons/download";
  import AudioPlayer from "#lib/components/shared/audio-player.svelte";
  import { Badge } from "#lib/components/ui/badge/index.js";
  import { Button } from "#lib/components/ui/button/index.js";
  import { Card } from "#lib/components/ui/card/index.js";
  import { COPY } from "#lib/constants.js";
  import type { Clip } from "#lib/takes/types.js";

  interface Props {
    clip: Clip | null;
    exported: boolean;
    onexport: () => void;
    tone?: "voice" | "music";
  }

  let { clip, exported, onexport, tone }: Props = $props();

  const copy = COPY.clip;
</script>

{#if clip}
  <Card class="shrink-0">
    <div class="flex items-center justify-between gap-4">
      <h2 class="truncate font-semibold">{clip.title}</h2>
      <div class="flex shrink-0 items-center gap-3">
        {#if exported}<Badge tone="success">{copy.exported}</Badge>{/if}
        <Button variant="outline" onclick={onexport}>
          <DownloadIcon />
          {copy.export}
        </Button>
      </div>
    </div>
    {#key clip.url}
      <AudioPlayer url={clip.url} peaks={clip.peaks} durationMs={clip.durationMs} {tone} />
    {/key}
  </Card>
{/if}
