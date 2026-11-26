<script lang="ts">
  import { ToggleGroup } from "bits-ui";

  interface Props {
    label: string;
    value: string;
    options: { value: string; label: string }[];
    onchange: (value: string) => void;
    disabled?: boolean;
  }

  let { label, value, options, onchange, disabled }: Props = $props();
</script>

<!-- Pressing the chosen option again reports nothing, which would leave none chosen. -->
<ToggleGroup.Root
  type="single"
  data-slot="segmented"
  aria-label={label}
  {disabled}
  bind:value={() => value, (next: string) => next && onchange(next)}
  class="inline-flex w-fit gap-1 rounded-md bg-muted p-1"
>
  {#each options as option (option.value)}
    <ToggleGroup.Item
      value={option.value}
      class="h-8 rounded-sm px-4 text-sm font-medium text-muted-foreground transition-colors outline-none hover:text-foreground focus-visible:ring-3 focus-visible:ring-ring/40 disabled:pointer-events-none disabled:opacity-60 data-[state=on]:bg-card data-[state=on]:text-secondary data-[state=on]:shadow-xs"
    >
      {option.label}
    </ToggleGroup.Item>
  {/each}
</ToggleGroup.Root>
