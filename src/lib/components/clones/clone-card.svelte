<script lang="ts">
  import PencilIcon from "@lucide/svelte/icons/pencil";
  import PlayIcon from "@lucide/svelte/icons/play";
  import TrashIcon from "@lucide/svelte/icons/trash-2";
  import XIcon from "@lucide/svelte/icons/x";
  import { goto } from "$app/navigation";
  import AudioPlayer from "#lib/components/shared/audio-player.svelte";
  import TitleEditor from "#lib/components/shared/title-editor.svelte";
  import { AlertDialog } from "#lib/components/ui/alert-dialog/index.js";
  import { Button } from "#lib/components/ui/button/index.js";
  import { Card } from "#lib/components/ui/card/index.js";
  import { clones } from "#lib/clones/store.svelte.js";
  import type { ClonedVoice } from "#lib/clones/types.js";
  import { COPY, MAX_VOICE_NAME_LENGTH, ROUTES } from "#lib/constants.js";
  import { models } from "#lib/models/store.svelte.js";
  import { formatDate } from "#lib/utils.js";
  import { voice } from "#lib/voice/store.svelte.js";

  let { clone }: { clone: ClonedVoice } = $props();

  let renaming = $state(false);
  let confirming = $state(false);

  const copy = COPY.clones;
  const id = $derived(clone.id);
  const url = $derived(clones.playing?.id === id ? clones.playing.url : null);
  const usable = $derived(!!models.cloning && models.isInstalled(models.cloning.id));

  async function use() {
    if (voice.useClone(id)) await goto(ROUTES.voice);
  }
</script>

<Card>
  <div class="flex items-center gap-4">
    <Button
      variant={url ? "outline" : "default"}
      size="round"
      aria-label={url ? copy.close : copy.play}
      onclick={() => (url ? clones.close() : clones.play(id))}
    >
      {#if url}<XIcon />{:else}<PlayIcon />{/if}
    </Button>
    <div class="min-w-0 flex-1">
      {#if renaming}
        <TitleEditor
          title={clone.name}
          maxlength={MAX_VOICE_NAME_LENGTH}
          oncancel={() => (renaming = false)}
          onsave={async (name: string) => {
            await clones.rename(id, name);
            renaming = false;
          }}
        />
      {:else}
        <h2 class="truncate font-semibold" title={clone.name}>{clone.name}</h2>
      {/if}
      <p class="truncate text-sm text-muted-foreground tabular-nums">
        {copy.details(copy.owners[clone.consent], formatDate(clone.createdMs))}
      </p>
    </div>
    {#if usable}<Button variant="outline" onclick={use}>{copy.use}</Button>{/if}
    <div class="flex shrink-0">
      <Button variant="ghost" size="icon" aria-label={copy.rename} title={copy.rename} onclick={() => (renaming = true)}>
        <PencilIcon />
      </Button>
      <Button variant="ghost" size="icon" aria-label={copy.delete} title={copy.delete} onclick={() => (confirming = true)}>
        <TrashIcon />
      </Button>
    </div>
  </div>
  {#if url}
    <AudioPlayer {url} peaks={clone.peaks} durationMs={clone.durationMs} autoplay />
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
    void clones.delete(id);
  }}
/>
