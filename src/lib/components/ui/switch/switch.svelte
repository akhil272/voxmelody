<script lang="ts">
  import { Switch as SwitchPrimitive } from "bits-ui";

  let { checked, label, onchange }: { checked: boolean; label: string; onchange: (checked: boolean) => void } = $props();
</script>

<label class="flex shrink-0 items-center gap-2.5 text-sm font-medium">
  {label}
  <SwitchPrimitive.Root
    data-slot="switch"
    {checked}
    onCheckedChange={onchange}
    aria-label={label}
    class="inline-flex h-6 w-10 items-center rounded-full bg-border p-0.5 transition-colors outline-none focus-visible:ring-3 focus-visible:ring-ring/40 data-[state=checked]:bg-primary"
  >
    <SwitchPrimitive.Thumb
      class="size-5 rounded-full bg-card shadow-xs transition-transform data-[state=checked]:translate-x-4"
    />
  </SwitchPrimitive.Root>
</label>
