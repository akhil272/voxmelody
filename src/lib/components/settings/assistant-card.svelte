<script lang="ts">
  import { isTauri } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { getAssistantCommand } from "#lib/assistant/api.js";
  import CopyButton from "#lib/components/shared/copy-button.svelte";
  import { Card } from "#lib/components/ui/card/index.js";
  import { COPY } from "#lib/constants.js";

  let command = $state("");

  const copy = COPY.assistant;

  onMount(async () => {
    if (isTauri()) command = (await getAssistantCommand()) ?? "";
  });
</script>

<Card>
  <div class="flex items-center justify-between gap-4">
    <div class="min-w-0">
      <h3 class="font-semibold">{copy.name}</h3>
      <p class="text-sm text-muted-foreground">{copy.description}</p>
    </div>
    <CopyButton value={command} label={copy.copy} variant="outline" size="default" />
  </div>
  {#if command}<code class="rounded-md bg-muted px-3 py-2 text-xs break-all select-text">{command}</code>{/if}
</Card>
