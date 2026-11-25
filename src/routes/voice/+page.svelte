<script lang="ts">
  import StudioPage from "#lib/components/shared/studio-page.svelte";
  import ClipCard from "#lib/components/voice/clip-card.svelte";
  import ScriptEditor from "#lib/components/voice/script-editor.svelte";
  import VoiceControls from "#lib/components/voice/voice-controls.svelte";
  import { COPY } from "#lib/constants.js";
</script>

<StudioPage id="voice">
  <div class="flex h-full flex-col gap-5 p-8">
    <h1 class="text-2xl font-semibold">{COPY.nav.voice}</h1>
    <div class="flex min-h-0 flex-1 gap-5">
      <ScriptEditor />
      <VoiceControls />
    </div>
    <ClipCard />
  </div>
</StudioPage>
