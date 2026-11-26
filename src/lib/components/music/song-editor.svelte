<script lang="ts">
  import PlusIcon from "@lucide/svelte/icons/plus";
  import Field from "#lib/components/shared/field.svelte";
  import TextEditor from "#lib/components/shared/text-editor.svelte";
  import { Button } from "#lib/components/ui/button/index.js";
  import { Textarea } from "#lib/components/ui/textarea/index.js";
  import { COPY, MAX_LYRICS_LENGTH, MAX_PROMPT_LENGTH, SONG_SECTIONS } from "#lib/constants.js";
  import { music } from "#lib/music/store.svelte.js";
  import { addSection } from "#lib/utils.js";

  const copy = COPY.music;

  let editor: HTMLElement;

  function add(section: (typeof SONG_SECTIONS)[number]["value"]) {
    music.lyrics = addSection(music.lyrics, section);
    editor.querySelector("textarea")?.focus();
  }
</script>

<div class="flex min-h-0 min-w-0 flex-1 flex-col gap-4">
  <Field label={copy.prompt}>
    <Textarea
      rows={2}
      aria-label={copy.prompt}
      aria-invalid={music.length > MAX_PROMPT_LENGTH}
      placeholder={copy.songPlaceholder}
      oninput={() => music.clearError()}
      bind:value={music.prompt}
    />
  </Field>
  <div class="flex items-center justify-between gap-2">
    <span class="text-sm font-medium">{copy.lyrics}</span>
    <div class="flex gap-4">
      {#each SONG_SECTIONS as { value, label } (value)}
        <Button variant="link" size="inline" onclick={() => add(value)}>
          <PlusIcon />
          {label}
        </Button>
      {/each}
    </div>
  </div>
  <div bind:this={editor} class="-mt-2 flex min-h-0 flex-1">
    <TextEditor
      bind:value={music.lyrics}
      label={copy.lyrics}
      placeholder={copy.lyricsPlaceholder}
      length={music.lyricsLength}
      limit={MAX_LYRICS_LENGTH}
      oninput={() => music.clearError()}
    />
  </div>
</div>
