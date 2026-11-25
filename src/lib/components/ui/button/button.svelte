<script lang="ts">
  import type { HTMLAnchorAttributes, HTMLButtonAttributes } from "svelte/elements";
  import { cn } from "#lib/utils.js";
  import { buttonVariants, type ButtonVariants } from "./variants.js";

  type Props = ButtonVariants &
    (({ href: string } & HTMLAnchorAttributes) | ({ href?: undefined } & HTMLButtonAttributes));

  let { class: className, variant, size, children, ...restProps }: Props = $props();

  const classes = $derived(cn(buttonVariants({ variant, size }), className));
</script>

{#if restProps.href !== undefined}
  <a data-slot="button" class={classes} draggable="false" {...restProps}>{@render children?.()}</a>
{:else}
  <button data-slot="button" type="button" class={classes} {...restProps}>{@render children?.()}</button>
{/if}
