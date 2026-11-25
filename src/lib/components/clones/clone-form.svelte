<script lang="ts">
  import { onMount } from "svelte";
  import SampleRecorder from "#lib/components/clones/sample-recorder.svelte";
  import Field from "#lib/components/shared/field.svelte";
  import OptionSelect from "#lib/components/shared/option-select.svelte";
  import { Button } from "#lib/components/ui/button/index.js";
  import { Card } from "#lib/components/ui/card/index.js";
  import { Checkbox } from "#lib/components/ui/checkbox/index.js";
  import { Input } from "#lib/components/ui/input/index.js";
  import { clones } from "#lib/clones/store.svelte.js";
  import type { CloneConsent } from "#lib/clones/types.js";
  import { CONSENT_OPTIONS, COPY, MAX_VOICE_NAME_LENGTH } from "#lib/constants.js";

  let { onclose }: { onclose: () => void } = $props();

  let name = $state("");
  let owner = $state<CloneConsent>("own");
  let agreed = $state(false);

  const copy = COPY.clones;
  const ready = $derived(!!clones.sample && name.trim() !== "" && agreed && !clones.busy && !clones.recording);

  function setOwner(value: string) {
    owner = value as CloneConsent;
    // Each statement needs its own agreement.
    agreed = false;
  }

  async function create() {
    if (await clones.create(name, owner)) onclose();
  }

  onMount(() => () => void clones.discard());
</script>

<Card class="gap-5">
  <h2 class="text-lg font-semibold">{copy.newVoice}</h2>
  <div class="grid grid-cols-2 gap-4">
    <Field label={copy.name}>
      <Input aria-label={copy.name} maxlength={MAX_VOICE_NAME_LENGTH} bind:value={name} />
    </Field>
    <Field label={copy.owner}>
      <OptionSelect
        label={copy.owner}
        value={owner}
        options={CONSENT_OPTIONS}
        disabled={clones.recording}
        onchange={setOwner}
      />
    </Field>
  </div>
  <SampleRecorder {owner} />
  <Checkbox bind:checked={agreed} label={copy.agreements[owner]} />
  {#if clones.error}
    <p role="alert" class="text-sm text-destructive">{copy.errors[clones.error]}</p>
  {/if}
  <div class="flex justify-end gap-2">
    <Button variant="outline" onclick={onclose}>{copy.cancel}</Button>
    <Button disabled={!ready} onclick={create}>{copy.create}</Button>
  </div>
</Card>
