<script lang="ts">
  import { AlertDialog as Primitive } from "bits-ui";
  import { buttonVariants } from "#lib/components/ui/button/index.js";

  interface Props {
    open: boolean;
    title: string;
    description: string;
    confirm: string;
    cancel: string;
    onconfirm: () => void;
  }

  let { open = $bindable(), title, description, confirm, cancel, onconfirm }: Props = $props();
</script>

<Primitive.Root bind:open>
  <Primitive.Portal>
    <Primitive.Overlay class="fixed inset-0 z-50 bg-foreground/40" />
    <Primitive.Content
      data-slot="alert-dialog"
      class="fixed top-1/2 left-1/2 z-50 flex w-full max-w-sm -translate-x-1/2 -translate-y-1/2 flex-col gap-2 rounded-xl border bg-card p-6 shadow-lg outline-none"
    >
      <Primitive.Title class="font-heading text-lg font-semibold">{title}</Primitive.Title>
      <Primitive.Description class="text-sm text-muted-foreground">{description}</Primitive.Description>
      <div class="mt-4 flex justify-end gap-2">
        <Primitive.Cancel class={buttonVariants({ variant: "outline" })}>{cancel}</Primitive.Cancel>
        <Primitive.Action class={buttonVariants({ variant: "destructive" })} onclick={onconfirm}>
          {confirm}
        </Primitive.Action>
      </div>
    </Primitive.Content>
  </Primitive.Portal>
</Primitive.Root>
