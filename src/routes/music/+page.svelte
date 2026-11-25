<script lang="ts">
  import MusicControls from "#lib/components/music/music-controls.svelte";
  import ClipCard from "#lib/components/shared/clip-card.svelte";
  import PageHeader from "#lib/components/shared/page-header.svelte";
  import StudioPage from "#lib/components/shared/studio-page.svelte";
  import TextEditor from "#lib/components/shared/text-editor.svelte";
  import { COPY, MAX_PROMPT_LENGTH } from "#lib/constants.js";
  import { music } from "#lib/music/store.svelte.js";
</script>

<StudioPage id="music">
  <div class="flex h-full flex-col gap-5 p-8">
    <PageHeader title={COPY.nav.music} />
    <div class="flex min-h-0 flex-1 gap-5">
      <TextEditor
        bind:value={music.prompt}
        label={COPY.music.prompt}
        placeholder={COPY.music.placeholder}
        length={music.length}
        limit={MAX_PROMPT_LENGTH}
        oninput={() => music.clearError()}
      />
      <MusicControls />
    </div>
    <ClipCard clip={music.clip} exported={music.exported} tone="music" onexport={() => music.export()} />
  </div>
</StudioPage>
