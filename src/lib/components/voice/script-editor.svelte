<script lang="ts">
  import { Textarea } from "#lib/components/ui/textarea/index.js";
  import { COPY, MAX_SCRIPT_LENGTH } from "#lib/constants.js";
  import { cn } from "#lib/utils.js";
  import { voice } from "#lib/voice/store.svelte.js";

  const copy = COPY.voice;
  const over = $derived(voice.length > MAX_SCRIPT_LENGTH);
</script>

<div class="flex min-h-0 flex-1 flex-col gap-2">
  <Textarea
    class="min-h-40 flex-1 p-4 text-base"
    aria-label={copy.script}
    aria-invalid={over}
    placeholder={COPY.home.voicePlaceholder}
    oninput={() => voice.clearError()}
    bind:value={voice.script}
  />
  <p class={cn("text-right text-xs tabular-nums", over ? "text-destructive" : "text-muted-foreground")}>
    {copy.count(voice.length, MAX_SCRIPT_LENGTH)}
  </p>
</div>
