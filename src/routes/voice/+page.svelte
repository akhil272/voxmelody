<script lang="ts">
  import ClipCard from "#lib/components/shared/clip-card.svelte";
  import PageHeader from "#lib/components/shared/page-header.svelte";
  import StudioPage from "#lib/components/shared/studio-page.svelte";
  import TextEditor from "#lib/components/shared/text-editor.svelte";
  import VoiceControls from "#lib/components/voice/voice-controls.svelte";
  import { COPY, MAX_SCRIPT_LENGTH } from "#lib/constants.js";
  import { voice } from "#lib/voice/store.svelte.js";
</script>

<StudioPage id="voice">
  <div class="flex h-full flex-col gap-5 p-8">
    <PageHeader title={COPY.nav.voice} />
    <div class="flex min-h-0 flex-1 gap-5">
      <TextEditor
        bind:value={voice.script}
        label={COPY.voice.script}
        placeholder={COPY.home.voicePlaceholder}
        length={voice.length}
        limit={MAX_SCRIPT_LENGTH}
        oninput={() => voice.clearError()}
      />
      <VoiceControls />
    </div>
    <ClipCard clip={voice.clip} exported={voice.exported} onexport={() => voice.export()} />
  </div>
</StudioPage>
