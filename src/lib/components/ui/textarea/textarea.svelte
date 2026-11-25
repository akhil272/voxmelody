<script lang="ts">
  import type { HTMLTextareaAttributes } from "svelte/elements";
  import { cn } from "#lib/utils.js";

  let { value = $bindable(), class: className, ...restProps }: HTMLTextareaAttributes = $props();
</script>

<textarea
  data-slot="textarea"
  class={cn(
    "w-full resize-none rounded-md border bg-card px-3 py-2 text-sm transition-colors outline-none select-text placeholder:text-muted-foreground focus-visible:border-ring focus-visible:ring-3 focus-visible:ring-ring/30 disabled:cursor-not-allowed disabled:opacity-70",
    className,
  )}
  bind:value
  {...restProps}></textarea>
