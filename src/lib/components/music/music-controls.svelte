<script lang="ts">
  import MusicAdvanced from "#lib/components/music/music-advanced.svelte";
  import MusicActions from "#lib/components/music/music-actions.svelte";
  import Field from "#lib/components/shared/field.svelte";
  import ModelField from "#lib/components/shared/model-field.svelte";
  import OptionSelect from "#lib/components/shared/option-select.svelte";
  import { Card } from "#lib/components/ui/card/index.js";
  import { Slider } from "#lib/components/ui/slider/index.js";
  import { BPM, COPY, LENGTH, MUSIC_KEYS, TEMPO_MODES } from "#lib/constants.js";
  import { music } from "#lib/music/store.svelte.js";
  import type { TempoMode } from "#lib/music/types.js";
  import { preferences } from "#lib/preferences/store.svelte.js";
  import { formatTime } from "#lib/utils.js";

  const copy = COPY.music;
  const custom = $derived(music.tempo === "custom");
</script>

<Card class="max-h-full w-72 shrink-0 gap-5 self-start">
  <div class="-mx-4 flex min-h-0 flex-col gap-5 overflow-y-auto px-4 [scrollbar-width:thin]">
    <ModelField kind="music" disabled={music.generating} />
    <Field label={copy.length} hint={formatTime(music.seconds)}>
      <Slider bind:value={music.seconds} {...LENGTH} label={copy.length} tone="music" disabled={music.generating} />
    </Field>
    <Field label={copy.tempo} hint={custom ? copy.bpm(music.bpm) : undefined}>
      <OptionSelect
        label={copy.tempo}
        value={music.tempo}
        options={TEMPO_MODES}
        disabled={music.generating}
        onchange={(mode: string) => (music.tempo = mode as TempoMode)}
      />
      {#if custom}
        <Slider bind:value={music.bpm} {...BPM} label={copy.tempo} tone="music" disabled={music.generating} />
      {/if}
    </Field>
    <Field label={copy.key}>
      <OptionSelect
        label={copy.key}
        value={music.key}
        options={MUSIC_KEYS}
        disabled={music.generating}
        onchange={(key: string) => (music.key = key)}
      />
    </Field>
    {#if preferences.advanced}<MusicAdvanced />{/if}
  </div>
  <MusicActions />
</Card>
