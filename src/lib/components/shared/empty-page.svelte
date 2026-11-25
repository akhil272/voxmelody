<script lang="ts">
  import type { LucideIcon } from "@lucide/svelte";
  import PagePanel from "#lib/components/shared/page-panel.svelte";

  interface Props {
    title: string;
    empty: string;
    description: string;
    icon: LucideIcon;
  }

  let { title, empty, description, icon: Icon }: Props = $props();
</script>

<PagePanel {title}>
  <span class="grid size-14 place-items-center rounded-full bg-accent text-primary"><Icon class="size-6" /></span>
  <div>
    <h2 class="text-lg font-semibold">{empty}</h2>
    <p class="mt-1 text-sm text-muted-foreground">{description}</p>
  </div>
</PagePanel>
