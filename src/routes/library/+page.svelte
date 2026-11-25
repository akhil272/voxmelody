<script lang="ts">
  import FolderIcon from "@lucide/svelte/icons/folder";
  import { onMount } from "svelte";
  import RecordingCard from "#lib/components/library/recording-card.svelte";
  import EmptyPage from "#lib/components/shared/empty-page.svelte";
  import { COPY } from "#lib/constants.js";
  import { library } from "#lib/library/store.svelte.js";

  onMount(() => {
    void library.refresh();
    return () => library.stop();
  });
</script>

<!-- Files may be moved while the app is in the background. -->
<svelte:window onfocus={() => library.refresh()} />

{#if library.recordings.length > 0}
  <div class="flex flex-col gap-3 p-8">
    <h1 class="mb-2 text-2xl font-semibold">{COPY.nav.library}</h1>
    {#each library.recordings as recording (recording.id)}
      <RecordingCard {recording} />
    {/each}
  </div>
{:else if library.loadError}
  <p role="alert" class="p-8 text-sm text-destructive">{COPY.library.errors[library.loadError]}</p>
{:else if library.loaded}
  <EmptyPage title={COPY.nav.library} {...COPY.pages.library} icon={FolderIcon} />
{/if}
