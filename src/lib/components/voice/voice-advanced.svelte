<script lang="ts">
  import Field from "#lib/components/shared/field.svelte";
  import { Slider } from "#lib/components/ui/slider/index.js";
  import { AUTO, COPY, PAUSE, VOICE_STEPS } from "#lib/constants.js";
  import { models } from "#lib/models/store.svelte.js";
  import { voice } from "#lib/voice/store.svelte.js";

  const copy = COPY.voice;
  const busy = $derived(voice.activity !== "idle");
  const threads = $derived({ min: AUTO, max: models.system?.cpuThreads ?? 1, step: 1 });
  // Each model family has its own controls.
  const supertonic = $derived(models.selected("voice")?.voice?.family === "supertonic");
</script>

{#if supertonic}
  <Field label={copy.steps} hint={voice.steps === AUTO ? copy.auto : String(voice.steps)}>
    <Slider bind:value={voice.steps} {...VOICE_STEPS} label={copy.steps} disabled={busy} />
  </Field>
{:else}
  <Field label={copy.pauses} hint={copy.speedValue(voice.pause)}>
    <Slider bind:value={voice.pause} {...PAUSE} label={copy.pauses} disabled={busy} />
  </Field>
{/if}
<Field label={copy.threads} hint={voice.threads === AUTO ? copy.auto : String(voice.threads)}>
  <Slider bind:value={voice.threads} {...threads} label={copy.threads} disabled={busy} />
</Field>
