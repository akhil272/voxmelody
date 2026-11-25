<script lang="ts">
  import { Button } from "#lib/components/ui/button/index.js";
  import { Card } from "#lib/components/ui/card/index.js";
  import { Progress } from "#lib/components/ui/progress/index.js";
  import { COPY } from "#lib/constants.js";
  import { models } from "#lib/models/store.svelte.js";

  const copy = COPY.location;
  const location = $derived(models.location);
  const moving = $derived(location?.movePercent ?? null);
</script>

<Card>
  <div class="flex items-center justify-between gap-4">
    <div class="min-w-0">
      <h3 class="font-semibold">{copy.folder}</h3>
      <p class="truncate text-sm text-muted-foreground select-text" title={location?.path}>
        {location?.path ?? COPY.system.unknown}
      </p>
    </div>
    <div class="flex shrink-0 gap-2">
      {#if location && !location.isDefault}
        <Button variant="outline" disabled={moving !== null} onclick={() => models.restoreLocation()}>
          {copy.restore}
        </Button>
      {/if}
      <Button variant="outline" disabled={!location || moving !== null} onclick={() => models.changeLocation()}>
        {location?.available === false ? copy.relink : copy.change}
      </Button>
    </div>
  </div>
  {#if moving !== null}
    <Progress value={moving} label={copy.folder} />
    <p class="text-sm text-muted-foreground tabular-nums">{copy.moving(moving)}</p>
  {:else if models.locationError}
    <p role="alert" class="text-sm text-destructive">{COPY.models.errors[models.locationError]}</p>
  {:else if location?.available === false}
    <p role="alert" class="text-sm text-warning">{copy.unavailable}</p>
  {/if}
</Card>
