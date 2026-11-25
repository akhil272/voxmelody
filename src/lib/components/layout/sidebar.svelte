<script lang="ts">
  import NavList from "#lib/components/layout/nav-list.svelte";
  import { COPY, FOOTER_NAV, MAIN_NAV } from "#lib/constants.js";
</script>

<nav
  aria-label={COPY.navigation}
  class="flex w-44 shrink-0 flex-col justify-between border-r bg-card/60 p-3 pt-6"
>
  <NavList items={MAIN_NAV} />
  <div class="border-t pt-3"><NavList items={FOOTER_NAV} /></div>
</nav>
