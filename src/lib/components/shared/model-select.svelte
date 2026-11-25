<script lang="ts">
  import type { LucideIcon } from "@lucide/svelte";
  import { Select, SelectContent, SelectItem, SelectTrigger } from "#lib/components/ui/select/index.js";

  interface Props {
    label: string;
    model: string;
    icon: LucideIcon;
    disabled?: boolean;
  }

  let { label, model, icon: Icon, disabled }: Props = $props();
</script>

<Select type="single" value={model} {disabled}>
  <SelectTrigger aria-label={label}>
    <Icon />
    {model}
  </SelectTrigger>
  <SelectContent>
    <SelectItem value={model} label={model} />
  </SelectContent>
</Select>
