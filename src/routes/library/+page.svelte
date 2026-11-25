<script lang="ts">
  import FolderIcon from "@lucide/svelte/icons/folder";
  import EmptyPage from "#lib/components/shared/empty-page.svelte";
  import { COPY } from "#lib/constants.js";
</script>

<EmptyPage title={COPY.nav.library} {...COPY.pages.library} icon={FolderIcon} />
