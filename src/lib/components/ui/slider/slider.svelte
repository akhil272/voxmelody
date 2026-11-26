<script lang="ts">
  import { Slider as SliderPrimitive } from "bits-ui";
  import { cn } from "#lib/utils.js";

  interface Props {
    value: number;
    min: number;
    max: number;
    step: number;
    label: string;
    disabled?: boolean;
    tone?: "voice" | "music";
  }

  let { value = $bindable(), min, max, step, label, disabled, tone = "voice" }: Props = $props();
</script>

<SliderPrimitive.Root
  type="single"
  data-slot="slider"
  bind:value
  {min}
  {max}
  {step}
  {disabled}
  class="relative flex h-5 w-full touch-none items-center data-disabled:opacity-60"
>
  <span class="h-1.5 w-full overflow-hidden rounded-full bg-muted">
    <SliderPrimitive.Range class={cn("absolute h-1.5 rounded-full", tone === "voice" ? "bg-primary" : "bg-secondary")} />
  </span>
  <SliderPrimitive.Thumb
    index={0}
    aria-label={label}
    class={cn(
      "block size-5 rounded-full border-2 bg-card shadow-xs transition-shadow outline-none focus-visible:ring-3",
      tone === "voice" ? "border-primary focus-visible:ring-ring/40" : "border-secondary focus-visible:ring-secondary/40",
    )}
  />
</SliderPrimitive.Root>
