<script lang="ts">
  import { cn } from "#lib/utils.js";

  let { class: className }: { class?: string } = $props();
</script>

<svg
  viewBox="0 0 142 80"
  class={cn("h-5 w-auto", className)}
  fill="none"
  stroke-width="21"
  stroke-linecap="round"
  stroke-linejoin="round"
  aria-hidden="true"
>
  <defs>
    <linearGradient id="vm-violet" x1="0" y1="0" x2="1" y2="1">
      <stop stop-color="var(--violet-bright)" />
      <stop offset="1" stop-color="var(--primary)" />
    </linearGradient>
    <linearGradient id="vm-cobalt" x1="0" y1="1" x2="1" y2="0">
      <stop stop-color="var(--secondary)" />
      <stop offset="1" stop-color="var(--cobalt-bright)" />
    </linearGradient>
  </defs>
  <path d="M13 14 40 62 69 14 90 46" stroke="url(#vm-violet)" />
  <path d="M78 66 105 18 130 54" stroke="url(#vm-cobalt)" />
  <circle cx="76" cy="66" r="13" fill="var(--secondary)" />
</svg>
