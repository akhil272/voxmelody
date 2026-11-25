<script lang="ts">
  import Field from "#lib/components/shared/field.svelte";
  import OptionSelect from "#lib/components/shared/option-select.svelte";
  import ModelField from "#lib/components/shared/model-field.svelte";
  import VoiceActions from "#lib/components/voice/voice-actions.svelte";
  import VoiceAdvanced from "#lib/components/voice/voice-advanced.svelte";
  import { Card } from "#lib/components/ui/card/index.js";
  import { Slider } from "#lib/components/ui/slider/index.js";
  import { COPY, SPEED } from "#lib/constants.js";
  import { preferences } from "#lib/preferences/store.svelte.js";
  import { toLanguageOptions } from "#lib/utils.js";
  import { voice } from "#lib/voice/store.svelte.js";

  const copy = COPY.voice;
  const busy = $derived(voice.activity !== "idle");
  const languages = $derived(toLanguageOptions(voice.available));
  const voices = $derived(
    voice.available
      .filter((option) => option.language === voice.voice?.language)
      .map(({ id, name }) => ({ value: id, label: name })),
  );
</script>

<Card class="max-h-full w-72 shrink-0 gap-5 self-start">
  <div class="-mx-4 flex min-h-0 flex-col gap-5 overflow-y-auto px-4 [scrollbar-width:thin]">
    <ModelField kind="voice" disabled={busy} />
    <Field label={copy.language}>
      <OptionSelect
        label={copy.language}
        value={voice.voice?.language ?? ""}
        options={languages}
        disabled={busy}
        onchange={(language: string) => voice.setLanguage(language)}
      />
    </Field>
    <Field label={copy.voice}>
      <OptionSelect
        label={copy.voice}
        value={voice.voice?.id ?? ""}
        options={voices}
        disabled={busy}
        onchange={(id: string) => voice.setVoice(id)}
      />
    </Field>
    <Field label={copy.speed} hint={copy.speedValue(voice.speed)}>
      <Slider bind:value={voice.speed} {...SPEED} label={copy.speed} disabled={busy} />
    </Field>
    {#if preferences.advanced}<VoiceAdvanced />{/if}
  </div>
  <VoiceActions />
</Card>
