<script lang="ts">
  import { fileLetter, strength, type Entry } from '$lib/password';
  import CopyButton from './copy-button.svelte';
  import Icon from './icon.svelte';
  import Stamp from './stamp.svelte';

  let {
    entries,
    total,
    weakCount,
    query = $bindable(''),
    selectedId,
    onselect,
    oncopy,
    onmove,
    oncopyselected
  }: {
    entries: Entry[];
    total: number;
    weakCount: number;
    query?: string;
    selectedId: number | null;
    onselect: (entry: Entry) => void;
    oncopy: (entry: Entry) => Promise<boolean>;
    onmove: (delta: number) => void;
    oncopyselected: () => void;
  } = $props();

  const LETTERS = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ'.split('');

  // Entries arrive sorted by site, so groups come out in drawer order.
  const groups = $derived.by(() => {
    const out: { letter: string; entries: Entry[] }[] = [];
    for (const entry of entries) {
      const letter = fileLetter(entry.site);
      const last = out.at(-1);
      if (last?.letter === letter) last.entries.push(entry);
      else out.push({ letter, entries: [entry] });
    }
    return out;
  });
  const filed = $derived(new Set(groups.map((g) => g.letter)));
  const index = $derived(filed.has('#') ? ['#', ...LETTERS] : LETTERS);
  // The thumb index earns its place once the drawer is long enough to scroll.
  const showIndex = $derived(entries.length >= 15);

  let list: HTMLDivElement;
  const sections: Record<string, HTMLElement> = {};
  let current = $state('');
  let focused = $state(false);

  function trackLetter() {
    let letter = groups[0]?.letter ?? '';
    for (const g of groups) {
      const el = sections[g.letter];
      if (el && el.offsetTop - list.scrollTop <= 1) letter = g.letter;
    }
    current = letter;
  }

  $effect(() => {
    void groups;
    if (list) trackLetter();
  });

  function jump(letter: string) {
    const el = sections[letter];
    if (el) list.scrollTo({ top: el.offsetTop, behavior: 'smooth' });
  }

  function onSearchKey(e: KeyboardEvent) {
    if (e.key === 'ArrowDown' || e.key === 'ArrowUp') {
      e.preventDefault();
      onmove(e.key === 'ArrowDown' ? 1 : -1);
    } else if (e.key === 'Enter') {
      e.preventDefault();
      oncopyselected();
    } else if (e.key === 'Escape' && query) {
      e.stopPropagation();
      query = '';
    }
  }

  // Keep the keyboard selection in view without jumping the drawer otherwise.
  $effect(() => {
    if (selectedId === null || !list) return;
    list.querySelector<HTMLElement>(`[data-entry="${selectedId}"]`)?.scrollIntoView({ block: 'nearest' });
  });
</script>

<section aria-label="Drawer" class="flex h-full min-h-0 flex-col border-r border-hair bg-paper">
  <div class="px-4 pt-4 pb-3">
    <div class="relative">
      <Icon name="search" class="pointer-events-none absolute top-1/2 left-3 size-4 -translate-y-1/2 text-muted" />
      <input
        type="search"
        aria-label="Search logins"
        placeholder="Search logins"
        autocomplete="off"
        spellcheck="false"
        bind:value={query}
        onkeydown={onSearchKey}
        onfocus={() => (focused = true)}
        onblur={() => (focused = false)}
        class="h-10 w-full rounded-lg border border-line bg-card pr-10 pl-9 text-sm text-ink transition-colors duration-150 outline-none placeholder:text-muted hover:border-active/50 focus:border-active focus:ring-3 focus:ring-active/20"
      />
      {#if !query && !focused}
        <kbd class="pointer-events-none absolute top-1/2 right-2.5 -translate-y-1/2">/</kbd>
      {/if}
    </div>
    <p class="mt-2.5 flex gap-2 text-xs text-muted tabular-nums" aria-live="polite">
      <span>{query.trim() ? `${entries.length} of ${total}` : `${total} ${total === 1 ? 'login' : 'logins'}`}</span>
      {#if weakCount > 0}
        <span aria-hidden="true">·</span>
        <span class="font-semibold text-weak">{weakCount} weak</span>
      {/if}
    </p>
  </div>

  <div class="flex min-h-0 flex-1">
    <div bind:this={list} onscroll={trackLetter} class="relative min-h-0 flex-1 overflow-y-auto pb-6">
      {#if total === 0}
        <p class="px-4 py-10 text-center text-sm text-muted">Nothing filed yet.</p>
      {:else if entries.length === 0}
        <div class="px-4 py-10 text-center">
          <p class="text-sm text-muted [overflow-wrap:anywhere]">No logins match “{query.trim()}”.</p>
          <button type="button" onclick={() => (query = '')} class="mt-2 text-sm font-semibold text-active underline underline-offset-4 hover:no-underline">
            Clear search
          </button>
        </div>
      {/if}

      {#each groups as group, gi (group.letter)}
        <section bind:this={sections[group.letter]} aria-label={`Filed under ${group.letter}`}>
          <!-- Guide card: tabs step across the drawer in thirds, like a real catalog. -->
          <h2 class="sticky top-0 z-10 flex h-8 items-end border-b-2 border-board bg-paper px-4">
            <span
              class="grid h-6 w-[52px] place-items-center rounded-t-md bg-board text-[13px] font-extrabold text-board-ink"
              style={`margin-left: calc((100% - 52px) * ${gi % 3} / 2)`}
            >
              {group.letter}
            </span>
          </h2>
          <ul>
            {#each group.entries as entry (entry.id)}
              {@const selected = entry.id === selectedId}
              {@const weak = strength(entry.password).score < 2}
              <li data-entry={entry.id} class={`flex h-11 scroll-mt-8 items-center border-b border-hair transition-colors duration-150 ${selected ? 'bg-select' : 'hover:bg-select/50'}`}>
                <button
                  type="button"
                  aria-current={selected ? 'true' : undefined}
                  onclick={() => onselect(entry)}
                  class="flex h-full min-w-0 flex-1 flex-col justify-center pl-4 text-left focus-visible:-outline-offset-2"
                >
                  <span class="flex min-w-0 items-center gap-2">
                    <span class={`truncate text-sm leading-tight font-semibold ${selected ? 'text-active' : ''}`}>{entry.site}</span>
                    {#if weak}<Stamp text="Weak" tone="weak" small />{/if}
                  </span>
                  <span class="truncate font-type text-[13px] leading-tight text-muted">{entry.username}</span>
                </button>
                <CopyButton label={`Copy password for ${entry.site}`} oncopy={() => oncopy(entry)} class="mr-1.5" />
              </li>
            {/each}
          </ul>
        </section>
      {/each}
    </div>

    {#if showIndex}
      <nav aria-label="Jump to letter" class="flex w-6 shrink-0 flex-col py-1">
        {#each index as letter}
          {@const has = filed.has(letter)}
          <button
            type="button"
            disabled={!has}
            aria-label={`Jump to ${letter}`}
            onclick={() => jump(letter)}
            class={`min-h-0 flex-1 text-[11px] leading-none font-bold transition-colors duration-150 ${current === letter ? 'text-active' : has ? 'text-ink hover:text-active' : 'text-muted/40'}`}
          >
            {letter}
          </button>
        {/each}
      </nav>
    {/if}
  </div>
</section>
