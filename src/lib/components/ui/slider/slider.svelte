<script lang="ts">
  import { Slider as SliderPrimitive } from "bits-ui";

  interface Props {
    value: number;
    min: number;
    max: number;
    step: number;
    label: string;
    disabled?: boolean;
  }

  let { value = $bindable(), min, max, step, label, disabled }: Props = $props();
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
    <SliderPrimitive.Range class="absolute h-1.5 rounded-full bg-primary" />
  </span>
  <SliderPrimitive.Thumb
    index={0}
    aria-label={label}
    class="block size-5 rounded-full border-2 border-primary bg-card shadow-xs transition-shadow outline-none focus-visible:ring-3 focus-visible:ring-ring/40"
  />
</SliderPrimitive.Root>
