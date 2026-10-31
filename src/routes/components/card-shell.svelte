<script lang="ts">
  import type { Snippet } from 'svelte';
  import Icon from './icon.svelte';
  import Stamp, { type StampState } from './stamp.svelte';

  let {
    title,
    sub,
    stamp = null,
    stampNote = '',
    error = '',
    tab = null,
    onback,
    children
  }: {
    title: string;
    sub?: Snippet;
    stamp?: StampState | null;
    /** Small typed line under the stamp, e.g. the clipboard countdown. */
    stampNote?: string;
    error?: string;
    /** Guide tab on the card's top edge, at the same third as its tab in the drawer. */
    tab?: { letter: string; pos: number } | null;
    /** Narrow windows show the card over the drawer; this returns to it. */
    onback?: () => void;
    children: Snippet;
  } = $props();
</script>

<article class={`relative w-full max-w-[640px] animate-rise border border-hair bg-card ${tab ? 'mt-6 rounded-b-lg border-t-2 border-t-board' : 'rounded-lg'}`}>
  {#if tab}
    <span
      aria-hidden="true"
      class="absolute -top-[26px] grid h-6 w-[52px] place-items-center rounded-t-md bg-board text-[13px] font-extrabold text-board-ink"
      style={`left: calc(32px + (100% - 64px - 52px) * ${tab.pos} / 2)`}
    >
      {tab.letter}
    </span>
  {/if}
  <header class="px-8 pt-7 pb-5 max-sm:px-5 max-sm:pt-5">
    {#if onback}
      <button
        type="button"
        onclick={onback}
        class="-ml-2 mb-3 inline-flex h-9 items-center gap-1.5 rounded-lg px-2 text-sm font-semibold text-muted hover:bg-select hover:text-ink min-[761px]:hidden"
      >
        <Icon name="arrow-left" />
        Drawer
      </button>
    {/if}
    <div class="flex items-start justify-between gap-4">
      <div class="min-w-0">
        <h1 class="text-[30px] leading-tight font-extrabold tracking-[-0.01em] [overflow-wrap:anywhere] max-sm:text-2xl">{title}</h1>
        {#if sub}
          <p class="mt-1 text-sm text-muted">{@render sub()}</p>
        {/if}
      </div>
      {#if stamp}
        <div class="flex flex-col items-end gap-2 pt-1">
          {#key stamp.id}
            <Stamp text={stamp.text} tone={stamp.tone} />
          {/key}
          {#if stampNote}
            <span class="font-type text-xs text-muted tabular-nums">{stampNote}</span>
          {/if}
        </div>
      {/if}
    </div>
  </header>

  <div class="h-0.5 bg-rule" aria-hidden="true"></div>

  {#if error}
    <p role="alert" class="mx-8 mt-4 text-sm font-semibold text-weak max-sm:mx-5">{error}</p>
  {/if}

  <div class="px-8 pt-2 pb-7 max-sm:px-5">
    {@render children()}
  </div>
</article>
