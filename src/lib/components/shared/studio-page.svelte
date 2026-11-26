<script lang="ts">
  import type { Snippet } from "svelte";
  import ModelCard from "#lib/components/models/model-card.svelte";
  import EmptyPage from "#lib/components/shared/empty-page.svelte";
  import { COPY, PACK_UI } from "#lib/constants.js";
  import { models } from "#lib/models/store.svelte.js";
  import type { ModelKind } from "#lib/models/types.js";

  let { id, children }: { id: ModelKind; children: Snippet } = $props();

  const installed = $derived(!!models.selected(id));
  const recommended = $derived(models.recommended(id));
  const page = $derived(COPY.pages[id]);
</script>

{#if installed}
  {@render children()}
{:else}
  <EmptyPage
    title={COPY.nav[id]}
    empty={page.setup}
    description={page.description}
    icon={PACK_UI[id].icon}
  >
    {#if recommended}<div class="mt-4 w-full max-w-lg"><ModelCard model={recommended} /></div>{/if}
  </EmptyPage>
{/if}
