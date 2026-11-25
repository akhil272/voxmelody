<script lang="ts">
  import type { Snippet } from "svelte";
  import { Card } from "#lib/components/ui/card/index.js";

  let { title, children }: { title: string; children: Snippet } = $props();
</script>

<div class="flex h-full flex-col gap-6 p-8">
  <h1 class="text-2xl font-semibold">{title}</h1>
  <Card class="flex-1 items-center justify-center text-center">{@render children()}</Card>
</div>
