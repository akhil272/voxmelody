<script lang="ts">
  import AudioLinesIcon from "@lucide/svelte/icons/audio-lines";
  import MicIcon from "@lucide/svelte/icons/mic";
  import PlayIcon from "@lucide/svelte/icons/play";
  import { goto } from "$app/navigation";
  import StudioCard from "#lib/components/home/studio-card.svelte";
  import ModelSelect from "#lib/components/shared/model-select.svelte";
  import { Button } from "#lib/components/ui/button/index.js";
  import { Textarea } from "#lib/components/ui/textarea/index.js";
  import { COPY, ROUTES } from "#lib/constants.js";
  import { models } from "#lib/models/store.svelte.js";
  import { voice } from "#lib/voice/store.svelte.js";

  const copy = COPY.home;
  const installed = $derived(models.packs.voice?.state === "installed");
  const previewing = $derived(voice.activity === "preview");

  async function generate() {
    await goto(ROUTES.voice);
    if (installed) await voice.generate();
  }
</script>

<StudioCard title={copy.voiceTitle} description={copy.voiceDescription} icon={MicIcon} tone="voice">
  <Textarea
    rows={1}
    aria-label={copy.voiceTitle}
    placeholder={copy.voicePlaceholder}
    oninput={() => voice.clearError()}
    bind:value={voice.script}
  />
  {#if voice.error}
    <p role="alert" class="text-sm text-destructive">{COPY.voice.errors[voice.error]}</p>
  {/if}
  <div class="flex gap-2">
    <ModelSelect label={copy.voiceModel} model={copy.voiceEngine} icon={AudioLinesIcon} />
    {#if previewing}
      <Button variant="outline" onclick={() => voice.cancel()}>{COPY.voice.cancel}</Button>
    {:else}
      <Button variant="outline" disabled={!installed || !voice.ready} onclick={() => voice.playPreview()}>
        <PlayIcon />
        {copy.preview}
      </Button>
    {/if}
    <Button class="flex-1" disabled={!voice.ready} onclick={generate}>{copy.generate}</Button>
  </div>
</StudioCard>
