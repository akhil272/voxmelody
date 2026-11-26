<script lang="ts">
  import FolderIcon from "@lucide/svelte/icons/folder";
  import { onMount } from "svelte";
  import RecordingCard from "#lib/components/library/recording-card.svelte";
  import EmptyPage from "#lib/components/shared/empty-page.svelte";
  import { Segmented } from "#lib/components/ui/segmented/index.js";
  import { COPY, DEFAULT_LIBRARY_FILTER, LIBRARY_FILTERS } from "#lib/constants.js";
  import { library } from "#lib/library/store.svelte.js";
  import type { LibraryFilter } from "#lib/library/types.js";
  import { filterRecordings } from "#lib/utils.js";

  let filter = $state<LibraryFilter>(DEFAULT_LIBRARY_FILTER);
  const shown = $derived(filterRecordings(library.recordings, filter));

  onMount(() => {
    void library.refresh();
    return () => library.stop();
  });
</script>

<!-- Files may be moved while the app is in the background. -->
<svelte:window onfocus={() => library.refresh()} />

{#if library.recordings.length > 0}
  <div class="flex flex-col gap-3 p-8">
    <div class="mb-2 flex flex-wrap items-center justify-between gap-3">
      <h1 class="text-2xl font-semibold">{COPY.nav.library}</h1>
      <Segmented
        label={COPY.library.filter}
        value={filter}
        options={LIBRARY_FILTERS}
        onchange={(value) => (filter = value as LibraryFilter)}
      />
    </div>
    {#each shown as recording (recording.id)}
      <RecordingCard {recording} />
    {:else}
      <p class="py-12 text-center text-sm text-muted-foreground">{COPY.library.noneOfKind}</p>
    {/each}
  </div>
{:else if library.loadError}
  <p role="alert" class="p-8 text-sm text-destructive">{COPY.library.errors[library.loadError]}</p>
{:else if library.loaded}
  <EmptyPage title={COPY.nav.library} {...COPY.pages.library} icon={FolderIcon} />
{/if}
