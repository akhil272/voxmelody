<script lang="ts">
  import LocationCard from "#lib/components/models/location-card.svelte";
  import ModelCard from "#lib/components/models/model-card.svelte";
  import SystemCard from "#lib/components/models/system-card.svelte";
  import PageSection from "#lib/components/shared/page-section.svelte";
  import { COPY, PACK_IDS } from "#lib/constants.js";
</script>

<div class="mx-auto flex max-w-4xl flex-col gap-6 p-8">
  <h1 class="text-2xl font-semibold">{COPY.nav.settings}</h1>
  <PageSection title={COPY.models.title}>
    {#each PACK_IDS as id (id)}
      <ModelCard {id} />
    {/each}
  </PageSection>
  <PageSection title={COPY.location.title}>
    <LocationCard />
  </PageSection>
  <PageSection title={COPY.system.title}>
    <SystemCard />
  </PageSection>
</div>
