<script lang="ts">
  import Field from "#lib/components/shared/field.svelte";
  import { Button } from "#lib/components/ui/button/index.js";
  import { Card } from "#lib/components/ui/card/index.js";
  import { Slider } from "#lib/components/ui/slider/index.js";
  import { COPY, INTRO, MUSIC_VOLUME, OUTRO } from "#lib/constants.js";
  import type { Recording } from "#lib/library/types.js";
  import { mix } from "#lib/mix/store.svelte.js";

  let { voice, music }: { voice: Recording; music: Recording } = $props();

  const copy = COPY.mix;
</script>

<Card class="w-72 shrink-0 gap-5 self-start">
  <Field label={copy.volume} hint={copy.percent(mix.volume)}>
    <Slider bind:value={mix.volume} {...MUSIC_VOLUME} label={copy.volume} disabled={mix.creating} />
  </Field>
  <Field label={copy.intro} hint={copy.seconds(mix.intro)}>
    <Slider bind:value={mix.intro} {...INTRO} label={copy.intro} disabled={mix.creating} />
  </Field>
  <Field label={copy.outro} hint={copy.seconds(mix.outro)}>
    <Slider bind:value={mix.outro} {...OUTRO} label={copy.outro} disabled={mix.creating} />
  </Field>
  <div class="flex flex-col gap-3">
    <Button disabled={mix.creating} onclick={() => mix.create(voice, music)}>
      {mix.creating ? copy.creating : copy.create}
    </Button>
    {#if mix.error}
      <p role="alert" class="text-sm text-destructive">{copy.errors[mix.error]}</p>
    {/if}
  </div>
</Card>
