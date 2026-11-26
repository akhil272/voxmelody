<script lang="ts">
  import CopyButton from "#lib/components/shared/copy-button.svelte";
  import { COPY } from "#lib/constants.js";
  import { models } from "#lib/models/store.svelte.js";
  import type { ModelInfo } from "#lib/models/types.js";
  import { formatBytes, formatMemory, formatMonth, summariseLanguages } from "#lib/utils.js";

  let { model }: { model: ModelInfo } = $props();

  const copy = COPY.models;
  const pack = $derived(models.packs[model.id]);
  const rows = $derived(
    [
      { label: copy.author, value: model.author },
      { label: copy.released, value: formatMonth(model.released) },
      { label: copy.licence, value: model.licence },
      { label: copy.size, value: pack && formatBytes(pack.totalBytes) },
      { label: copy.languages, value: summariseLanguages(model.languages) },
      { label: copy.memory, value: formatMemory(model.recommendedMemory) },
      { label: copy.parameters, value: model.parameters },
      { label: copy.precision, value: model.precision },
    ].filter(({ value }) => value),
  );
</script>

<dl class="grid grid-cols-4 gap-x-4 gap-y-2">
  {#each rows as { label, value } (label)}
    <div class="min-w-0">
      <dt class="text-xs text-muted-foreground">{label}</dt>
      <dd class="truncate text-sm font-medium tabular-nums" title={value}>{value}</dd>
    </div>
  {/each}
</dl>
<div class="flex items-center gap-3">
  <span class="min-w-0 flex-1 truncate text-xs text-muted-foreground select-text">{model.pageUrl}</span>
  <CopyButton value={model.pageUrl} label={copy.copyLink} />
</div>
