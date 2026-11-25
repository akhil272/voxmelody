<script lang="ts">
  import Field from "#lib/components/shared/field.svelte";
  import OptionSelect from "#lib/components/shared/option-select.svelte";
  import { Card } from "#lib/components/ui/card/index.js";
  import { COPY } from "#lib/constants.js";
  import { library } from "#lib/library/store.svelte.js";
  import type { Recording } from "#lib/library/types.js";
  import { mix } from "#lib/mix/store.svelte.js";
  import { toRecordingOptions } from "#lib/utils.js";

  let { voice, music }: { voice: Recording; music: Recording } = $props();

  const copy = COPY.mix;
</script>

<Card class="min-w-0 flex-1 gap-5 self-start">
  <Field label={copy.voice}>
    <OptionSelect
      label={copy.voice}
      value={String(voice.id)}
      options={toRecordingOptions(library.voices)}
      disabled={mix.creating}
      onchange={(id: string) => (mix.voiceId = Number(id))}
    />
  </Field>
  <Field label={copy.music}>
    <OptionSelect
      label={copy.music}
      value={String(music.id)}
      options={toRecordingOptions(library.tracks)}
      disabled={mix.creating}
      onchange={(id: string) => (mix.musicId = Number(id))}
    />
  </Field>
</Card>
