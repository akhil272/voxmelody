<script lang="ts">
  import DownloadIcon from "@lucide/svelte/icons/download";
  import PencilIcon from "@lucide/svelte/icons/pencil";
  import PlayIcon from "@lucide/svelte/icons/play";
  import TrashIcon from "@lucide/svelte/icons/trash-2";
  import XIcon from "@lucide/svelte/icons/x";
  import TitleEditor from "#lib/components/library/title-editor.svelte";
  import AudioPlayer from "#lib/components/shared/audio-player.svelte";
  import { AlertDialog } from "#lib/components/ui/alert-dialog/index.js";
  import { Badge } from "#lib/components/ui/badge/index.js";
  import { Button } from "#lib/components/ui/button/index.js";
  import { Card } from "#lib/components/ui/card/index.js";
  import { COPY, PACK_UI } from "#lib/constants.js";
  import { library } from "#lib/library/store.svelte.js";
  import type { Recording } from "#lib/library/types.js";
  import { formatDate, formatTime } from "#lib/utils.js";

  let { recording }: { recording: Recording } = $props();

  let renaming = $state(false);
  let confirming = $state(false);

  const copy = COPY.library;
  const id = $derived(recording.id);
  const ui = $derived(PACK_UI[recording.kind]);
  const url = $derived(library.playing?.id === id ? library.playing.value : null);
  const error = $derived(library.error?.id === id ? library.error.value : null);
  const details = $derived(
    copy.details(ui.title, formatTime(recording.durationMs / 1000), formatDate(recording.createdMs)),
  );
</script>

<Card>
  <div class="flex items-center gap-4">
    <Button
      variant={url ? "outline" : ui.button}
      size="round"
      aria-label={url ? copy.close : copy.play}
      disabled={!recording.available}
      onclick={() => (url ? library.stop() : library.play(id))}
    >
      {#if url}<XIcon />{:else}<PlayIcon />{/if}
    </Button>
    <div class="min-w-0 flex-1">
      {#if renaming}
        <TitleEditor
          title={recording.title}
          oncancel={() => (renaming = false)}
          onsave={async (title: string) => {
            await library.rename(id, title);
            renaming = false;
          }}
        />
      {:else}
        <h2 class="truncate font-semibold" title={recording.title}>{recording.title}</h2>
      {/if}
      <p class="truncate text-sm text-muted-foreground tabular-nums">{details}</p>
    </div>
    {#if !recording.available}
      <Badge tone="warning">{copy.missing}</Badge>
      <Button variant="outline" onclick={() => library.relink(id)}>{copy.relink}</Button>
    {:else if library.exportedId === id}
      <Badge tone="success">{COPY.clip.exported}</Badge>
    {/if}
    <div class="flex shrink-0">
      {#if recording.available}
        <Button variant="ghost" size="icon" aria-label={copy.export} title={copy.export} onclick={() => library.export(recording)}>
          <DownloadIcon />
        </Button>
      {/if}
      <Button variant="ghost" size="icon" aria-label={copy.rename} title={copy.rename} onclick={() => (renaming = true)}>
        <PencilIcon />
      </Button>
      <Button variant="ghost" size="icon" aria-label={copy.delete} title={copy.delete} onclick={() => (confirming = true)}>
        <TrashIcon />
      </Button>
    </div>
  </div>
  {#if error}
    <p role="alert" class="text-sm text-destructive">{copy.errors[error]}</p>
  {/if}
  {#if url}
    <AudioPlayer {url} peaks={recording.peaks} durationMs={recording.durationMs} tone={ui.tone} autoplay />
  {/if}
</Card>

<AlertDialog
  bind:open={confirming}
  title={copy.deleteTitle}
  description={copy.deleteDescription}
  confirm={copy.delete}
  cancel={copy.keep}
  onconfirm={() => {
    confirming = false;
    void library.delete(id);
  }}
/>
