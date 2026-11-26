<script lang="ts">
  import { Button } from "#lib/components/ui/button/index.js";
  import { Progress } from "#lib/components/ui/progress/index.js";
  import { COPY } from "#lib/constants.js";
  import { music } from "#lib/music/store.svelte.js";
  import { toPercent } from "#lib/utils.js";

  const copy = COPY.music;
  const percent = $derived(toPercent(music.progress, 1));
</script>

<div class="flex flex-col gap-3">
  {#if music.generating}
    <Progress value={percent} label={copy.generating} tone="music" />
    <p class="flex justify-between text-sm text-muted-foreground tabular-nums">
      <span aria-live="polite">{copy.generating}</span>
      {copy.percent(percent)}
    </p>
    <Button variant="outline" onclick={() => music.cancel()}>{copy.cancel}</Button>
  {:else}
    <Button variant="secondary" disabled={!music.ready} onclick={() => music.generate()}>{copy.generate}</Button>
  {/if}
  {#if music.error}
    <p role="alert" class="text-sm text-destructive">{copy.errors[music.error]}</p>
  {/if}
</div>
