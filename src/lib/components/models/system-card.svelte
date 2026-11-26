<script lang="ts">
  import { onMount } from "svelte";
  import { Card } from "#lib/components/ui/card/index.js";
  import { COPY } from "#lib/constants.js";
  import { models } from "#lib/models/store.svelte.js";
  import { formatBytes, formatMemory } from "#lib/utils.js";

  const copy = COPY.system;
  const system = $derived(models.system);
  const rows = $derived([
    { label: copy.memory, value: system && formatMemory(system.memoryBytes) },
    { label: copy.processor, value: system && copy.threads(system.cpuThreads) },
    { label: copy.graphics, value: models.graphics },
    { label: copy.freeSpace, value: system?.freeDiskBytes != null && formatBytes(system.freeDiskBytes) },
  ]);

  onMount(() => void models.loadGraphics());
</script>

<Card>
  <dl class="grid grid-cols-4 gap-4">
    {#each rows as { label, value } (label)}
      <div>
        <dt class="text-xs text-muted-foreground">{label}</dt>
        <dd class="font-medium tabular-nums">{value || copy.unknown}</dd>
      </div>
    {/each}
  </dl>
</Card>
