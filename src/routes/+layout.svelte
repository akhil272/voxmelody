<script lang="ts">
  import "./layout.css";
  import Sidebar from "#lib/components/layout/sidebar.svelte";
  import TitleBar from "#lib/components/layout/title-bar.svelte";
  import { onMount } from "svelte";
  import { voice } from "#lib/voice/store.svelte.js";
  import { music } from "#lib/music/store.svelte.js";
  import { models } from "#lib/models/store.svelte.js";
  import type { LayoutProps } from "./$types";

  let { children }: LayoutProps = $props();

  onMount(() => {
    void models.init();
    void music.init();
    void voice.init();
  });
</script>

<div class="flex h-full flex-col">
  <TitleBar />
  <div class="flex min-h-0 flex-1">
    <Sidebar />
    <main class="min-w-0 flex-1 overflow-y-auto">{@render children()}</main>
  </div>
</div>
