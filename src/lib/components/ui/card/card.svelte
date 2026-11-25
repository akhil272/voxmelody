<script lang="ts">
  import type { HTMLAttributes } from "svelte/elements";
  import { cn } from "#lib/utils.js";

  let { class: className, children, ...restProps }: HTMLAttributes<HTMLElement> = $props();
</script>

<section
  data-slot="card"
  class={cn("flex flex-col gap-3 rounded-xl border bg-card p-4 text-card-foreground shadow-xs", className)}
  {...restProps}
>
  {@render children?.()}
</section>
