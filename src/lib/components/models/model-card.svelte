<script lang="ts">
  import { AlertDialog } from "#lib/components/ui/alert-dialog/index.js";
  import ModelDetails from "#lib/components/models/model-details.svelte";
  import FavouriteButton from "#lib/components/models/favourite-button.svelte";
  import { Badge } from "#lib/components/ui/badge/index.js";
  import { Button } from "#lib/components/ui/button/index.js";
  import { Card } from "#lib/components/ui/card/index.js";
  import { Progress } from "#lib/components/ui/progress/index.js";
  import { COPY, PACK_UI } from "#lib/constants.js";
  import { models } from "#lib/models/store.svelte.js";
  import type { ModelInfo } from "#lib/models/types.js";
  import { cn, formatBytes, hasEnoughMemory, toPercent } from "#lib/utils.js";

  let { model, detailed = false }: { model: ModelInfo; detailed?: boolean } = $props();

  let confirming = $state(false);

  const copy = COPY.models;
  const id = $derived(model.id);
  const ui = $derived(PACK_UI[model.kind]);
  const recommended = $derived(models.recommended(model.kind)?.id === id);
  const Icon = $derived(ui.icon);
  const pack = $derived(models.packs[id]);
  const started = $derived(pack?.state === "downloading" || pack?.state === "paused");
  const moving = $derived((models.location?.movePercent ?? null) !== null);
  const lowMemory = $derived(
    pack && models.system && !hasEnoughMemory(models.system.memoryBytes, model.recommendedMemory),
  );
</script>

<Card class="text-left">
  <div class="flex items-start gap-4">
    <span class="grid size-11 shrink-0 place-items-center rounded-md bg-accent">
      <Icon class={cn("size-5", ui.tone === "voice" ? "text-primary" : "text-secondary")} />
    </span>
    <div class="min-w-0 flex-1">
      <h3 class="font-semibold">{model.name}</h3>
      <p class="text-sm text-muted-foreground">{copy.descriptions[id]}</p>
    </div>
    {#if recommended && pack?.state === "available"}<Badge tone="success">{copy.recommended}</Badge>{/if}
    {#if pack && pack.state !== "available"}
      <Badge tone={pack.state === "installed" ? "success" : "neutral"}>{copy.states[pack.state]}</Badge>
    {/if}
    {#if detailed}<FavouriteButton {id} />{/if}
  </div>

  {#if detailed}<ModelDetails {model} />{/if}
  {#if pack && started}
    <Progress value={toPercent(pack.downloadedBytes, pack.totalBytes)} label={model.name} tone={ui.tone} />
  {/if}
  {#if model.restricted}<p class="text-sm text-warning">{copy.restricted}</p>{/if}
  {#if pack?.error}
    <p role="alert" class="text-sm text-destructive">{copy.errors[pack.error]}</p>
  {:else if lowMemory && pack?.state !== "installed"}
    <p class="text-sm text-warning">{copy.lowMemory}</p>
  {/if}

  <div class="flex items-center justify-between gap-4">
    <span class="text-sm text-muted-foreground tabular-nums">
      {#if pack && started}
        {copy.progress(formatBytes(pack.downloadedBytes), formatBytes(pack.totalBytes))}
      {:else if pack}
        {formatBytes(pack.state === "available" ? pack.downloadBytes : pack.totalBytes)}
      {/if}
    </span>
    <div class="flex gap-2">
      {#if pack && pack.state !== "available"}
        <Button variant="outline" disabled={moving} onclick={() => (confirming = true)}>
          {started ? copy.cancel : copy.remove}
        </Button>
      {/if}
      {#if pack?.state === "downloading"}
        <Button variant={ui.button} disabled={moving} onclick={() => models.pause(id)}>{copy.pause}</Button>
      {:else if pack?.state !== "installed"}
        <Button variant={ui.button} disabled={!pack || moving} onclick={() => models.download(id)}>
          {pack?.error ? copy.retry : pack?.state === "paused" ? copy.resume : copy.download}
        </Button>
      {/if}
    </div>
  </div>
</Card>

<AlertDialog
  bind:open={confirming}
  title={started ? copy.cancelTitle : copy.removeTitle}
  description={started ? copy.cancelDescription : copy.removeDescription}
  confirm={started ? copy.discard : copy.remove}
  cancel={copy.keep}
  onconfirm={() => {
    confirming = false;
    void models.remove(id);
  }}
/>
