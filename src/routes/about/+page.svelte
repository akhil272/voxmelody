<script lang="ts">
  import Logo from "#lib/components/brand/logo.svelte";
  import Wordmark from "#lib/components/brand/wordmark.svelte";
  import PagePanel from "#lib/components/shared/page-panel.svelte";
  import { COPY } from "#lib/constants.js";
</script>

<PagePanel title={COPY.nav.about}>
  <Logo class="h-16" />
  <Wordmark class="text-3xl" />
  <p class="text-muted-foreground">{COPY.tagline}</p>
  <p class="text-sm">{COPY.about.privacy}</p>
  <p class="text-sm text-muted-foreground">{COPY.about.licence}</p>
</PagePanel>
