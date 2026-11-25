<script lang="ts">
  import { Progress as ProgressPrimitive } from "bits-ui";
  import { cn } from "#lib/utils.js";

  interface Props {
    value: number;
    label: string;
    tone?: "voice" | "music";
  }

  let { value, label, tone = "voice" }: Props = $props();
</script>

<ProgressPrimitive.Root
  data-slot="progress"
  {value}
  max={100}
  aria-label={label}
  class="h-2 w-full overflow-hidden rounded-full bg-muted"
>
  <div
    class={cn("h-full rounded-full transition-[width]", tone === "voice" ? "bg-primary" : "bg-secondary")}
    style:width="{value}%"
  ></div>
</ProgressPrimitive.Root>
