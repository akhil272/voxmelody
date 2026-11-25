<script lang="ts">
  import SettingsIcon from "@lucide/svelte/icons/settings";
  import EmptyPage from "#lib/components/shared/empty-page.svelte";
  import { COPY } from "#lib/constants.js";
</script>

<EmptyPage title={COPY.nav.settings} {...COPY.pages.settings} icon={SettingsIcon} />
