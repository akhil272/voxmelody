<script lang="ts">
  import { onMount } from "svelte";
  import { Button } from "#lib/components/ui/button/index.js";
  import { Card } from "#lib/components/ui/card/index.js";
  import { COPY } from "#lib/constants.js";
  import { library } from "#lib/library/store.svelte.js";

  const copy = COPY.saveFolder;
  const folder = $derived(library.saveFolder);

  onMount(() => void library.loadSaveFolder());
</script>

<Card>
  <div class="flex items-center justify-between gap-4">
    <div class="min-w-0">
      <h3 class="font-semibold">{copy.folder}</h3>
      <p class="truncate text-sm text-muted-foreground select-text" title={folder?.path}>
        {folder?.path ?? COPY.system.unknown}
      </p>
    </div>
    <div class="flex shrink-0 gap-2">
      {#if folder && !folder.isDefault}
        <Button variant="outline" onclick={() => library.restoreSaveFolder()}>{COPY.location.restore}</Button>
      {/if}
      <Button variant="outline" disabled={!folder} onclick={() => library.changeSaveFolder()}>
        {COPY.location.change}
      </Button>
    </div>
  </div>
  {#if library.saveFolderError}
    <p role="alert" class="text-sm text-destructive">{COPY.library.errors[library.saveFolderError]}</p>
  {:else if folder?.available === false}
    <p role="alert" class="text-sm text-warning">{copy.unavailable}</p>
  {:else}
    <p class="text-sm text-muted-foreground">{copy.description}</p>
  {/if}
</Card>
