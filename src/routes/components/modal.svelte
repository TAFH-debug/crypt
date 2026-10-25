<script lang="ts">
  import type { Snippet } from 'svelte';
  import { onMount } from 'svelte';
  import Button from './button.svelte';
  import Icon from './icon.svelte';

  let { title, onclose, children }: { title: string; onclose: () => void; children: Snippet } = $props();

  let dialog: HTMLDialogElement;

  // Native dialog gives focus trap, Escape to close and inert background.
  onMount(() => dialog.showModal());

  function closeOnBackdrop(e: MouseEvent) {
    if (e.target === dialog) dialog.close();
  }
</script>

<dialog
  bind:this={dialog}
  {onclose}
  onclick={closeOnBackdrop}
  class="m-auto w-full max-w-md animate-pop rounded-3xl bg-white text-zinc-900 shadow-2xl ring-1 ring-black/5 backdrop:bg-zinc-900/40 backdrop:backdrop-blur-sm dark:bg-zinc-900 dark:text-zinc-100 dark:ring-white/10"
>
  <div class="relative overflow-hidden">
    <div class="pointer-events-none absolute -top-24 -right-16 h-48 w-48 rounded-full bg-accent-2/20 blur-3xl"></div>
    <div class="relative flex items-center justify-between px-6 pt-5">
      <h2 class="text-lg font-bold tracking-tight">{title}</h2>
      <Button variant="ghost" class="p-1.5" aria-label="Close" onclick={() => dialog.close()}>
        <Icon name="close" />
      </Button>
    </div>
    <div class="relative px-6 pt-4 pb-6">
      {@render children()}
    </div>
  </div>
</dialog>
