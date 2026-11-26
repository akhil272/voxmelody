<script lang="ts">
  import ModelCatalog from "#lib/components/models/model-catalog.svelte";
  import ModelOverview from "#lib/components/models/model-overview.svelte";
  import PageHeader from "#lib/components/shared/page-header.svelte";
  import { COPY } from "#lib/constants.js";
  import { preferences } from "#lib/preferences/store.svelte.js";
</script>

<div class="mx-auto flex max-w-4xl flex-col gap-6 p-8">
  <PageHeader title={COPY.nav.models} />
  {#if preferences.advanced}
    <ModelCatalog />
  {:else}
    <ModelOverview />
  {/if}
</div>
