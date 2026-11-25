<script lang="ts">
  import ModelCard from "#lib/components/models/model-card.svelte";
  import EmptyPage from "#lib/components/shared/empty-page.svelte";
  import { COPY, PACK_UI } from "#lib/constants.js";
  import { models } from "#lib/models/store.svelte.js";
  import type { PackId } from "#lib/models/types.js";

  let { id }: { id: PackId } = $props();

  const installed = $derived(models.packs[id]?.state === "installed");
  const page = $derived(COPY.pages[id]);
</script>

<EmptyPage
  title={COPY.nav[id]}
  empty={installed ? page.empty : page.setup}
  description={page.description}
  icon={PACK_UI[id].icon}
>
  {#if !installed}
    <div class="mt-4 w-full max-w-lg"><ModelCard {id} /></div>
  {/if}
</EmptyPage>
