<script lang="ts">
  import MusicIcon from "@lucide/svelte/icons/music";
  import EmptyPage from "#lib/components/shared/empty-page.svelte";
  import { COPY } from "#lib/constants.js";
</script>

<EmptyPage title={COPY.nav.music} {...COPY.pages.music} icon={MusicIcon} />
