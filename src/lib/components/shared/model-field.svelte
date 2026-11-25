<script lang="ts">
  import Field from "#lib/components/shared/field.svelte";
  import ModelSelect from "#lib/components/shared/model-select.svelte";
  import { COPY } from "#lib/constants.js";
  import { models } from "#lib/models/store.svelte.js";
  import type { ModelKind } from "#lib/models/types.js";
  import { preferences } from "#lib/preferences/store.svelte.js";

  let { kind, disabled }: { kind: ModelKind; disabled?: boolean } = $props();
</script>

<!-- With one model there is nothing to choose. -->
{#if preferences.advanced || models.installed(kind).length > 1}
  <Field label={COPY.models.model}><ModelSelect {kind} {disabled} /></Field>
{/if}
