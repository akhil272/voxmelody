<script lang="ts">
  import ModelCard from "#lib/components/models/model-card.svelte";
  import ModelFilters from "#lib/components/models/model-filters.svelte";
  import { COPY, DEFAULT_FILTER } from "#lib/constants.js";
  import { models } from "#lib/models/store.svelte.js";
  import { preferences } from "#lib/preferences/store.svelte.js";
  import { filterModels } from "#lib/utils.js";

  let filter = $state({ ...DEFAULT_FILTER });

  const results = $derived(
    filterModels(
      models.catalog,
      filter,
      (id) => models.packs[id]?.totalBytes ?? 0,
      (id) => preferences.isFavourite(id),
    ),
  );
</script>

<div class="flex flex-col gap-2"><ModelFilters bind:filter /></div>
<div class="flex flex-col gap-3">
  {#each results as model (model.id)}
    <ModelCard {model} detailed />
  {:else}
    <p class="py-10 text-center text-sm text-muted-foreground">{COPY.models.noResults}</p>
  {/each}
</div>
