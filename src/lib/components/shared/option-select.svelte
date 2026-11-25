<script lang="ts">
  import { Select, SelectContent, SelectItem, SelectTrigger } from "#lib/components/ui/select/index.js";

  interface Props {
    label: string;
    value: string;
    options: { value: string; label: string }[];
    onchange: (value: string) => void;
    disabled?: boolean;
  }

  let { label, value, options, onchange, disabled }: Props = $props();

  const selected = $derived(options.find((option) => option.value === value));
</script>

<Select type="single" {value} {disabled} items={options} onValueChange={onchange}>
  <SelectTrigger class="w-full" aria-label={label}>{selected?.label}</SelectTrigger>
  <SelectContent>
    {#each options as option (option.value)}
      <SelectItem value={option.value} label={option.label} />
    {/each}
  </SelectContent>
</Select>
