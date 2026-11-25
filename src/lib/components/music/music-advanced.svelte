<script lang="ts">
  import Field from "#lib/components/shared/field.svelte";
  import OptionSelect from "#lib/components/shared/option-select.svelte";
  import { Input } from "#lib/components/ui/input/index.js";
  import { Slider } from "#lib/components/ui/slider/index.js";
  import { COPY, CREATIVITY, GUIDANCE, MUSIC_STEPS, PROCESSORS } from "#lib/constants.js";
  import { music } from "#lib/music/store.svelte.js";
  import type { Processor } from "#lib/music/types.js";
  import { isSeed } from "#lib/utils.js";

  const copy = COPY.music;
</script>

<Field label={copy.creativity} hint={music.creativity.toFixed(2)}>
  <Slider bind:value={music.creativity} {...CREATIVITY} label={copy.creativity} tone="music" disabled={music.generating} />
</Field>
<Field label={copy.steps} hint={String(music.steps)}>
  <Slider
    bind:value={() => music.steps, (steps: number) => music.setSteps(steps)}
    {...MUSIC_STEPS}
    label={copy.steps}
    tone="music"
    disabled={music.generating}
  />
</Field>
{#if music.model?.music?.guidance}
  <Field label={copy.guidance} hint={music.guidance.toFixed(1)}>
    <Slider bind:value={music.guidance} {...GUIDANCE} label={copy.guidance} tone="music" disabled={music.generating} />
  </Field>
{/if}
<Field label={copy.seed}>
  <Input
    aria-label={copy.seed}
    aria-invalid={!isSeed(music.seed)}
    class="aria-invalid:border-destructive"
    inputmode="numeric"
    placeholder={copy.random}
    disabled={music.generating}
    bind:value={music.seed}
  />
</Field>
<Field label={copy.processor}>
  <OptionSelect
    label={copy.processor}
    value={music.processor}
    options={PROCESSORS}
    disabled={music.generating}
    onchange={(processor: string) => (music.processor = processor as Processor)}
  />
</Field>
