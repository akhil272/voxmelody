<script lang="ts">
  import { Button } from "#lib/components/ui/button/index.js";
  import type { ButtonVariants } from "#lib/components/ui/button/variants.js";
  import { COPY } from "#lib/constants.js";

  interface Props {
    value: string;
    label: string;
    variant?: ButtonVariants["variant"];
    size?: ButtonVariants["size"];
  }

  let { value, label, variant = "link", size = "inline" }: Props = $props();

  let copied = $state(false);

  async function copy() {
    await navigator.clipboard.writeText(value);
    copied = true;
  }
</script>

<Button {variant} {size} disabled={!value} onclick={copy}>{copied ? COPY.models.copied : label}</Button>
