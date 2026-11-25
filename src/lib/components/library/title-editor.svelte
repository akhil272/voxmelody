<script lang="ts">
  import { Input } from "#lib/components/ui/input/index.js";
  import { COPY, MAX_TITLE_LENGTH } from "#lib/constants.js";

  let { title, onsave, oncancel }: { title: string; onsave: (title: string) => void; oncancel: () => void } = $props();

  // svelte-ignore state_referenced_locally
  let draft = $state(title);
  let finished = false;

  function finish(save: boolean) {
    // Removing a focused field also fires blur.
    if (finished) return;
    finished = true;
    const next = draft.trim();
    if (save && next && next !== title) onsave(next);
    else oncancel();
  }

  function focus(form: HTMLFormElement) {
    form.querySelector("input")?.select();
  }
</script>

<form
  class="min-w-0 flex-1"
  onsubmit={(event) => {
    event.preventDefault();
    finish(true);
  }}
  {@attach focus}
>
  <Input
    aria-label={COPY.library.name}
    maxlength={MAX_TITLE_LENGTH}
    onblur={() => finish(true)}
    onkeydown={(event: KeyboardEvent) => event.key === "Escape" && finish(false)}
    bind:value={draft}
  />
</form>
