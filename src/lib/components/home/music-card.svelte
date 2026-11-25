<script lang="ts">
  import MusicIcon from "@lucide/svelte/icons/music";
  import { goto } from "$app/navigation";
  import StudioCard from "#lib/components/home/studio-card.svelte";
  import ModelSelect from "#lib/components/shared/model-select.svelte";
  import { Button } from "#lib/components/ui/button/index.js";
  import { Textarea } from "#lib/components/ui/textarea/index.js";
  import { COPY, ROUTES } from "#lib/constants.js";
  import { models } from "#lib/models/store.svelte.js";
  import { music } from "#lib/music/store.svelte.js";

  const copy = COPY.home;

  async function generate() {
    await goto(ROUTES.music);
    if (models.packs.music?.state === "installed") await music.generate();
  }
</script>

<StudioCard title={copy.musicTitle} description={copy.musicDescription} icon={MusicIcon} tone="music">
  <Textarea
    rows={1}
    aria-label={copy.musicTitle}
    placeholder={COPY.music.placeholder}
    oninput={() => music.clearError()}
    bind:value={music.prompt}
  />
  {#if music.error}
    <p role="alert" class="text-sm text-destructive">{COPY.music.errors[music.error]}</p>
  {/if}
  <div class="flex gap-2">
    <ModelSelect label={copy.musicModel} model={copy.musicEngine} icon={MusicIcon} />
    <Button variant="secondary" class="flex-1" disabled={!music.ready} onclick={generate}>{copy.generateMusic}</Button>
  </div>
</StudioCard>
