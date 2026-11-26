<script lang="ts">
  import ModelCard from "#lib/components/models/model-card.svelte";
  import PageSection from "#lib/components/shared/page-section.svelte";
  import { MODEL_KINDS, PACK_UI } from "#lib/constants.js";
  import { models } from "#lib/models/store.svelte.js";
  import type { ModelKind } from "#lib/models/types.js";

  /** The recommended model, and any others already downloaded or in progress. */
  function shown(kind: ModelKind) {
    const recommended = models.recommended(kind);
    return models.catalog.filter(
      (model) =>
        model.kind === kind && (model === recommended || (models.packs[model.id]?.state ?? "available") !== "available"),
    );
  }
</script>

{#each MODEL_KINDS as kind (kind)}
  <PageSection title={PACK_UI[kind].title}>
    {#each shown(kind) as model (model.id)}
      <ModelCard {model} />
    {/each}
  </PageSection>
{/each}
