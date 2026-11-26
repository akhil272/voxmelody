<script lang="ts">
  import BlendIcon from "@lucide/svelte/icons/blend";
  import { onMount } from "svelte";
  import MixControls from "#lib/components/mix/mix-controls.svelte";
  import MixSources from "#lib/components/mix/mix-sources.svelte";
  import ClipCard from "#lib/components/shared/clip-card.svelte";
  import EmptyPage from "#lib/components/shared/empty-page.svelte";
  import { Button } from "#lib/components/ui/button/index.js";
  import { COPY, ROUTES } from "#lib/constants.js";
  import { library } from "#lib/library/store.svelte.js";
  import { mix } from "#lib/mix/store.svelte.js";
  import { findRecording } from "#lib/utils.js";

  const voice = $derived(findRecording(library.voices, mix.voiceId));
  const music = $derived(findRecording(library.tracks, mix.musicId));

  onMount(() => void library.refresh());
</script>

<!-- Files may be moved while the app is in the background. -->
<svelte:window onfocus={() => library.refresh()} />

{#if voice && music}
  <div class="flex min-h-full flex-col gap-5 p-8">
    <h1 class="text-2xl font-semibold">{COPY.nav.mix}</h1>
    <div class="flex flex-1 gap-5">
      <MixSources {voice} {music} />
      <MixControls {voice} {music} />
    </div>
    <ClipCard clip={mix.clip} exported={mix.exported} onexport={() => mix.export()} />
  </div>
{:else if library.loadError}
  <p role="alert" class="p-8 text-sm text-destructive">{COPY.library.errors[library.loadError]}</p>
{:else if library.loaded}
  <EmptyPage title={COPY.nav.mix} {...COPY.pages.mix} icon={BlendIcon}>
    <div class="mt-4 flex gap-3">
      {#if !voice}<Button href={ROUTES.voice}>{COPY.mix.addVoice}</Button>{/if}
      {#if !music}<Button href={ROUTES.music} variant="secondary">{COPY.mix.addMusic}</Button>{/if}
    </div>
  </EmptyPage>
{/if}
