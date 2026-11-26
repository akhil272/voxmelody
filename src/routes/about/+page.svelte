<script lang="ts">
  import CoffeeIcon from "@lucide/svelte/icons/coffee";
  import { getVersion } from "@tauri-apps/api/app";
  import { onMount } from "svelte";
  import SocialIcon from "#lib/components/about/social-icon.svelte";
  import Logo from "#lib/components/brand/logo.svelte";
  import Wordmark from "#lib/components/brand/wordmark.svelte";
  import { Button } from "#lib/components/ui/button/index.js";
  import { Card } from "#lib/components/ui/card/index.js";
  import { COPY } from "#lib/constants.js";
  import { ABOUT_PHOTO, SOCIAL_LINKS, SUPPORT_URL } from "#lib/constants/links.js";
  import { openExternal } from "#lib/utils/links.js";

  const copy = COPY.about;
  let version = $state("");

  onMount(() => {
    getVersion().then((value) => (version = value), () => undefined);
  });
</script>

<div class="mx-auto flex max-w-4xl flex-col gap-10 p-8">
  <section class="flex flex-col items-center gap-3 pt-4 text-center">
    <Logo class="h-16" />
    <Wordmark class="text-3xl" />
    <p class="text-muted-foreground">{COPY.tagline}</p>
    <p class="text-sm text-muted-foreground">
      {copy.licence}{#if version}&ensp;·&ensp;{copy.version(version)}{/if}
    </p>
  </section>

  <section class="grid gap-4 sm:grid-cols-3">
    {#each copy.promises as promise (promise.title)}
      <Card class="gap-1">
        <h2 class="font-semibold">{promise.title}</h2>
        <p class="text-sm text-pretty text-muted-foreground">{promise.text}</p>
      </Card>
    {/each}
  </section>

  <section class="grid items-center gap-8 border-t pt-10 md:grid-cols-[minmax(0,1fr)_14rem]">
    <div class="flex flex-col gap-4">
      <p class="text-xs font-medium tracking-[0.14em] text-muted-foreground uppercase">{copy.me.eyebrow}</p>
      <h2 class="text-3xl leading-tight font-semibold text-balance">
        {copy.me.title}
        <br />
        <span class="text-muted-foreground">{copy.me.subtitle}</span>
      </h2>
      {#each copy.me.paragraphs as paragraph (paragraph)}
        <p class="max-w-xl text-pretty text-muted-foreground">{paragraph}</p>
      {/each}
    </div>
    <img
      src={ABOUT_PHOTO}
      alt={copy.me.photo}
      width="640"
      height="640"
      class="aspect-[4/5] w-full max-w-56 justify-self-center rounded-2xl object-cover shadow-sm"
      draggable="false"
    />
  </section>

  <Card class="gap-4 bg-accent p-6 sm:flex-row sm:items-center sm:justify-between">
    <div class="flex flex-col gap-1">
      <h2 class="text-xl font-semibold">{copy.support.title}</h2>
      <p class="text-sm text-pretty text-muted-foreground">{copy.support.text}</p>
    </div>
    <Button size="lg" class="shrink-0" onclick={() => openExternal(SUPPORT_URL)}>
      <CoffeeIcon />
      {copy.support.action}
    </Button>
  </Card>

  <section class="grid gap-4 border-t pt-10 md:grid-cols-[14rem_minmax(0,1fr)] md:gap-10">
    <h2 class="text-2xl font-semibold">{copy.why.title}</h2>
    <div class="flex max-w-2xl flex-col gap-4 text-pretty text-muted-foreground">
      {#each copy.why.paragraphs as paragraph (paragraph)}
        <p>{paragraph}</p>
      {/each}
    </div>
  </section>

  <footer class="flex flex-col gap-3 border-t pt-8">
    <p class="text-sm font-medium">{copy.follow}</p>
    <div class="flex flex-wrap gap-2">
      {#each SOCIAL_LINKS as link (link.label)}
        <Button variant="outline" aria-label={copy.followOn(link.label)} onclick={() => openExternal(link.href)}>
          <SocialIcon icon={link.icon} />
          {link.label}
        </Button>
      {/each}
    </div>
  </footer>
</div>
