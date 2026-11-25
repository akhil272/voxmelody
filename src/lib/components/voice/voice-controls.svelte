<script lang="ts">
  import Field from "#lib/components/shared/field.svelte";
  import OptionSelect from "#lib/components/shared/option-select.svelte";
  import VoiceActions from "#lib/components/voice/voice-actions.svelte";
  import { Card } from "#lib/components/ui/card/index.js";
  import { Slider } from "#lib/components/ui/slider/index.js";
  import { COPY, SPEED, VOICE_LANGUAGES } from "#lib/constants.js";
  import { voice } from "#lib/voice/store.svelte.js";

  const copy = COPY.voice;
  const busy = $derived(voice.activity !== "idle");
  const voices = $derived(
    voice.voices
      .filter((option) => option.language === voice.voice?.language)
      .map(({ id, name }) => ({ value: id, label: name })),
  );
</script>

<Card class="w-72 shrink-0 gap-5 self-start">
  <Field label={copy.language}>
    <OptionSelect
      label={copy.language}
      value={voice.voice?.language ?? VOICE_LANGUAGES[0].value}
      options={VOICE_LANGUAGES}
      disabled={busy}
      onchange={(language: string) => voice.setLanguage(language)}
    />
  </Field>
  <Field label={copy.voice}>
    <OptionSelect
      label={copy.voice}
      value={voice.voiceId}
      options={voices}
      disabled={busy}
      onchange={(id: string) => voice.setVoice(id)}
    />
  </Field>
  <Field label={copy.speed} hint={copy.speedValue(voice.speed)}>
    <Slider bind:value={voice.speed} {...SPEED} label={copy.speed} disabled={busy} />
  </Field>
  <VoiceActions />
</Card>
