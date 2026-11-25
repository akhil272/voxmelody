<script lang="ts">
  import MicIcon from "@lucide/svelte/icons/mic";
  import MusicIcon from "@lucide/svelte/icons/music";
  import { waveBand, wavePath } from "#lib/utils.js";

  const WAVE_LINES = 14;
  const WAVE_SHAPE = { width: 560, baseline: 145, cycles: 1.5 };
</script>

<div class="pointer-events-none absolute inset-y-0 right-0 w-3/5" aria-hidden="true">
  <svg viewBox="0 0 560 270" preserveAspectRatio="none" class="size-full" fill="none">
    <defs>
      <linearGradient id="vm-wave" gradientUnits="userSpaceOnUse" x1="0" x2="560">
        <stop stop-color="var(--teal-bright)" stop-opacity="0" />
        <stop offset="0.35" stop-color="var(--teal-bright)" />
        <stop offset="1" stop-color="var(--secondary)" />
      </linearGradient>
    </defs>
    {#each { length: WAVE_LINES }, index (index)}
      <path
        d={wavePath({ ...WAVE_SHAPE, amplitude: 88 - index * 5, phase: 2.6 + index * 0.06 })}
        stroke="url(#vm-wave)"
        stroke-opacity={0.6 - index * 0.03}
      />
    {/each}
    <path
      d={waveBand({ ...WAVE_SHAPE, amplitude: 88, phase: 2.6 }, { ...WAVE_SHAPE, amplitude: 45, phase: 3.9 })}
      fill="url(#vm-wave)"
      fill-opacity="0.14"
    />
  </svg>
  <span
    class="absolute top-[18%] left-[48%] grid size-13 place-items-center rounded-full bg-card/90 text-primary shadow-md"
  >
    <MusicIcon class="size-6" />
  </span>
  <span
    class="absolute top-[40%] left-[66%] grid size-17 place-items-center rounded-full bg-card/90 text-secondary shadow-md"
  >
    <MicIcon class="size-8" />
  </span>
</div>
