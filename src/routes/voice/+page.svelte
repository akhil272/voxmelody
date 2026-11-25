<script lang="ts">
  import MicIcon from "@lucide/svelte/icons/mic";
  import EmptyPage from "#lib/components/shared/empty-page.svelte";
  import { COPY } from "#lib/constants.js";
</script>

<EmptyPage title={COPY.nav.voice} {...COPY.pages.voice} icon={MicIcon} />
