<script lang="ts">
  import type { HTMLInputAttributes } from "svelte/elements";
  import { cn } from "#lib/utils.js";

  type Props = Omit<HTMLInputAttributes, "value"> & { value: string };

  let { value = $bindable(), class: className, ...restProps }: Props = $props();
</script>

<input
  data-slot="input"
  class={cn(
    "h-9 w-full rounded-md border bg-card px-3 text-sm transition-colors outline-none select-text placeholder:text-muted-foreground focus-visible:border-ring focus-visible:ring-3 focus-visible:ring-ring/30 disabled:cursor-not-allowed disabled:opacity-70",
    className,
  )}
  bind:value
  {...restProps}
/>
