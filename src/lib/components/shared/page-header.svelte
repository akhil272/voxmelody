<script lang="ts">
  import { Switch } from "#lib/components/ui/switch/index.js";
  import { COPY } from "#lib/constants.js";
  import { preferences } from "#lib/preferences/store.svelte.js";

  let { title }: { title: string } = $props();
</script>

<header class="flex items-center justify-between gap-4">
  <h1 class="text-2xl font-semibold">{title}</h1>
  <Switch
    label={COPY.models.advanced}
    checked={preferences.advanced}
    onchange={(checked: boolean) => preferences.setAdvanced(checked)}
  />
</header>
