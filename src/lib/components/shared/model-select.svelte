<script lang="ts">
  import OptionSelect from "#lib/components/shared/option-select.svelte";
  import { COPY } from "#lib/constants.js";
  import { models } from "#lib/models/store.svelte.js";
  import type { ModelKind } from "#lib/models/types.js";
  import { preferences } from "#lib/preferences/store.svelte.js";
  import { toModelOptions } from "#lib/utils.js";

  let { kind, disabled }: { kind: ModelKind; disabled?: boolean } = $props();

  const installed = $derived(models.installed(kind));
  const selected = $derived(models.selected(kind));
</script>

{#if selected}
  <OptionSelect
    label={COPY.models.model}
    value={selected.id}
    options={toModelOptions(installed)}
    {disabled}
    onchange={(id: string) => preferences.setModel(kind, id)}
  />
{/if}
