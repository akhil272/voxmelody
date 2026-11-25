<script lang="ts">
  import CheckIcon from "@lucide/svelte/icons/check";
  import { Checkbox as CheckboxPrimitive } from "bits-ui";

  let { checked = $bindable(), label }: { checked: boolean; label: string } = $props();
</script>

<label class="flex items-start gap-2.5 text-sm">
  <CheckboxPrimitive.Root
    data-slot="checkbox"
    bind:checked
    aria-label={label}
    class="grid size-5 shrink-0 place-items-center rounded-sm border bg-card text-primary-foreground transition-colors outline-none focus-visible:ring-3 focus-visible:ring-ring/40 data-[state=checked]:border-primary data-[state=checked]:bg-primary"
  >
    {#if checked}<CheckIcon class="size-3.5" />{/if}
  </CheckboxPrimitive.Root>
  {label}
</label>
