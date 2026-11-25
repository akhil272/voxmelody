<script lang="ts">
  import StarIcon from "@lucide/svelte/icons/star";
  import OptionSelect from "#lib/components/shared/option-select.svelte";
  import { Button } from "#lib/components/ui/button/index.js";
  import { Input } from "#lib/components/ui/input/index.js";
  import { ALL, COPY, KIND_OPTIONS, SIZE_OPTIONS, SORT_OPTIONS } from "#lib/constants.js";
  import { models } from "#lib/models/store.svelte.js";
  import type { ModelFilter, ModelSize, ModelSort } from "#lib/models/types.js";

  let { filter = $bindable() }: { filter: ModelFilter } = $props();

  const copy = COPY.models;
  const licences = $derived([
    { value: ALL, label: copy.allLicences },
    ...[...new Set(models.catalog.map(({ licence }) => licence))].map((licence) => ({ value: licence, label: licence })),
  ]);
</script>

<div class="flex gap-2">
  <Input class="h-10" aria-label={copy.search} placeholder={copy.search} bind:value={filter.search} />
  <Button
    variant={filter.favourites ? "default" : "outline"}
    aria-pressed={filter.favourites}
    onclick={() => (filter.favourites = !filter.favourites)}
  >
    <StarIcon />
    {copy.favourites}
  </Button>
</div>
<div class="grid grid-cols-4 gap-2">
  <OptionSelect
    label={copy.kindFilter.all}
    value={filter.kind}
    options={KIND_OPTIONS}
    onchange={(kind: string) => (filter.kind = kind as ModelFilter["kind"])}
  />
  <OptionSelect
    label={copy.licence}
    value={filter.licence}
    options={licences}
    onchange={(licence: string) => (filter.licence = licence)}
  />
  <OptionSelect
    label={copy.size}
    value={filter.size}
    options={SIZE_OPTIONS}
    onchange={(size: string) => (filter.size = size as ModelSize)}
  />
  <OptionSelect
    label={copy.sort}
    value={filter.sort}
    options={SORT_OPTIONS}
    onchange={(sort: string) => (filter.sort = sort as ModelSort)}
  />
</div>
