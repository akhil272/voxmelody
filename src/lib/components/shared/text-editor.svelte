<script lang="ts">
  import { Textarea } from "#lib/components/ui/textarea/index.js";
  import { COPY } from "#lib/constants.js";
  import { cn } from "#lib/utils.js";

  interface Props {
    value: string;
    label: string;
    placeholder: string;
    length: number;
    limit: number;
    oninput: () => void;
  }

  let { value = $bindable(), label, placeholder, length, limit, oninput }: Props = $props();

  const over = $derived(length > limit);
</script>

<div class="flex min-h-0 flex-1 flex-col gap-2">
  <Textarea
    class="min-h-40 flex-1 p-4 text-base"
    aria-label={label}
    aria-invalid={over}
    {placeholder}
    {oninput}
    bind:value
  />
  <p class={cn("text-right text-xs tabular-nums", over ? "text-destructive" : "text-muted-foreground")}>
    {COPY.editor.count(length, limit)}
  </p>
</div>
