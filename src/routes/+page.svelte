<script lang="ts">
  import ArrowRightIcon from "@lucide/svelte/icons/arrow-right";
  import InfoIcon from "@lucide/svelte/icons/info";
  import MusicIcon from "@lucide/svelte/icons/music";
  import HeroWave from "#lib/components/brand/hero-wave.svelte";
  import Wordmark from "#lib/components/brand/wordmark.svelte";
  import StudioCard from "#lib/components/home/studio-card.svelte";
  import VoiceCard from "#lib/components/home/voice-card.svelte";
  import ModelSelect from "#lib/components/shared/model-select.svelte";
  import { Button } from "#lib/components/ui/button/index.js";
  import { COPY, ROUTES } from "#lib/constants.js";

  const copy = COPY.home;
</script>

<div class="flex min-h-full flex-col gap-4 px-5 pb-5">
  <section class="relative flex min-h-64 flex-1 items-center overflow-hidden px-4">
    <HeroWave />
    <div class="relative">
      <h1 class="text-2xl font-semibold">
        {copy.welcome}
        <Wordmark class="block text-[42px] leading-tight" />
      </h1>
      <p class="mt-1 max-w-72 text-muted-foreground">{copy.description}</p>
      <Button href={ROUTES.voice} size="lg" class="mt-5">
        {copy.start}
        <ArrowRightIcon />
      </Button>
    </div>
  </section>

  <div class="grid grid-cols-2 gap-4">
    <VoiceCard />

    <StudioCard title={copy.musicTitle} description={copy.musicDescription} icon={MusicIcon} tone="music">
      <ModelSelect label={copy.musicModel} model={copy.musicEngine} icon={MusicIcon} disabled />
      <Button variant="secondary" class="mt-auto w-full" disabled>{copy.generateMusic}</Button>
    </StudioCard>
  </div>

  <aside class="flex items-center justify-between rounded-lg border bg-muted px-4 py-2.5 text-xs">
    <span class="flex items-center gap-2.5">
      <InfoIcon class="size-4 text-primary" />
      {copy.privacy}
    </span>
    <Button href={ROUTES.about} variant="link" size="inline">
      {copy.learnMore}
      <ArrowRightIcon />
    </Button>
  </aside>
</div>
