<script lang="ts">
  import { COPY } from "#lib/constants.js";
  import { cn } from "#lib/utils.js";

  let { class: className }: { class?: string } = $props();
</script>

<span class={cn("font-heading font-bold tracking-tight", className)}>
  {COPY.appNameLead}<span
    class="bg-linear-to-r from-secondary to-primary to-35% bg-clip-text text-transparent">{COPY.appNameAccent}</span
  >
</span>
