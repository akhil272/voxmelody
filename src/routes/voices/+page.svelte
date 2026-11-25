<script lang="ts">
  import AudioLinesIcon from "@lucide/svelte/icons/audio-lines";
  import { onMount } from "svelte";
  import CloneCard from "#lib/components/clones/clone-card.svelte";
  import CloneForm from "#lib/components/clones/clone-form.svelte";
  import ModelCard from "#lib/components/models/model-card.svelte";
  import EmptyPage from "#lib/components/shared/empty-page.svelte";
  import { Button } from "#lib/components/ui/button/index.js";
  import { clones } from "#lib/clones/store.svelte.js";
  import { COPY } from "#lib/constants.js";
  import { models } from "#lib/models/store.svelte.js";

  let creating = $state(false);

  const copy = COPY.clones;
  const model = $derived(models.cloning);
  const installed = $derived(!!model && models.isInstalled(model.id));

  onMount(() => {
    void clones.refresh();
    return () => clones.close();
  });
</script>

{#if model && clones.loaded}
  {#if clones.voices.length === 0 && !creating && !clones.error}
    <EmptyPage title={COPY.nav.voices} {...COPY.pages.voices} icon={AudioLinesIcon}>
      {#if installed}
        <Button class="mt-4" onclick={() => (creating = true)}>{copy.newVoice}</Button>
      {:else}
        <div class="mt-4 w-full max-w-lg"><ModelCard {model} /></div>
      {/if}
    </EmptyPage>
  {:else}
    <div class="flex flex-col gap-3 p-8">
      <header class="mb-2 flex h-10 items-center justify-between gap-4">
        <h1 class="text-2xl font-semibold">{COPY.nav.voices}</h1>
        {#if installed && !creating}<Button onclick={() => (creating = true)}>{copy.newVoice}</Button>{/if}
      </header>
      {#if creating}
        <CloneForm onclose={() => (creating = false)} />
      {:else}
        {#if clones.error}
          <p role="alert" class="text-sm text-destructive">{copy.errors[clones.error]}</p>
        {/if}
        {#if !installed}<ModelCard {model} />{/if}
        {#each clones.voices as clone (clone.id)}
          <CloneCard {clone} />
        {/each}
      {/if}
    </div>
  {/if}
{/if}
