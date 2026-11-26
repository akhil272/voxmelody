<script lang="ts">
  import Field from "#lib/components/shared/field.svelte";
  import OptionSelect from "#lib/components/shared/option-select.svelte";
  import { Slider } from "#lib/components/ui/slider/index.js";
  import { COPY, LENGTH_MODES, SONG_LANGUAGES, SONG_LENGTH } from "#lib/constants.js";
  import { music } from "#lib/music/store.svelte.js";
  import type { LengthMode } from "#lib/music/types.js";
  import { formatTime } from "#lib/utils.js";

  const copy = COPY.music;
  const custom = $derived(music.songLength === "custom");
</script>

<Field label={copy.language}>
  <OptionSelect
    label={copy.language}
    value={music.language}
    options={SONG_LANGUAGES}
    disabled={music.generating}
    onchange={(language: string) => (music.language = language)}
  />
</Field>
<Field label={copy.length} hint={custom ? formatTime(music.songSeconds) : undefined}>
  <OptionSelect
    label={copy.length}
    value={music.songLength}
    options={LENGTH_MODES}
    disabled={music.generating}
    onchange={(mode: string) => (music.songLength = mode as LengthMode)}
  />
  {#if custom}
    <Slider bind:value={music.songSeconds} {...SONG_LENGTH} label={copy.length} tone="music" disabled={music.generating} />
  {/if}
</Field>
