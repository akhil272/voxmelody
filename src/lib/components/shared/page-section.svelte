<script lang="ts">
  import type { Snippet } from "svelte";

  let { title, children }: { title: string; children: Snippet } = $props();
</script>

<section class="flex flex-col gap-3">
  <h2 class="text-lg font-semibold">{title}</h2>
  {@render children()}
</section>
