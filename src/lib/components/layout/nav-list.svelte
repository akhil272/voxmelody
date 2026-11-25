<script lang="ts">
  import { page } from "$app/state";
  import type { NavItem } from "#lib/constants.js";
  import { cn, isActiveRoute } from "#lib/utils.js";

  let { items }: { items: NavItem[] } = $props();
</script>

<ul class="flex flex-col gap-1.5">
  {#each items as { href, label, icon: Icon } (href)}
    {@const active = isActiveRoute(page.url.pathname, href)}
    <li>
      <a
        {href}
        draggable="false"
        aria-current={active ? "page" : undefined}
        class={cn(
          "flex h-10 items-center gap-4 rounded-md px-3 text-sm transition-colors outline-none hover:bg-accent focus-visible:ring-3 focus-visible:ring-ring/40 [&_svg]:size-5",
          active && "bg-primary text-primary-foreground hover:bg-primary",
        )}
      >
        <Icon />
        {label}
      </a>
    </li>
  {/each}
</ul>
