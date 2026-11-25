<script lang="ts">
  import StarIcon from "@lucide/svelte/icons/star";
  import { Button } from "#lib/components/ui/button/index.js";
  import { COPY } from "#lib/constants.js";
  import { preferences } from "#lib/preferences/store.svelte.js";
  import { cn } from "#lib/utils.js";

  let { id }: { id: string } = $props();

  const favourite = $derived(preferences.isFavourite(id));
  const label = $derived(favourite ? COPY.models.unfavourite : COPY.models.favourite);
</script>

<Button
  variant="ghost"
  size="icon"
  aria-label={label}
  aria-pressed={favourite}
  title={label}
  onclick={() => preferences.toggleFavourite(id)}
>
  <StarIcon class={cn(favourite && "fill-warning text-warning")} />
</Button>
