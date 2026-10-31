<script lang="ts">
  import type { Entry } from '$lib/password';
  import Button from './button.svelte';
  import CardShell from './card-shell.svelte';
  import Icon from './icon.svelte';
  import Stamp, { type StampState } from './stamp.svelte';

  let {
    total,
    weak,
    stamp,
    error,
    onadd,
    onopen
  }: {
    total: number;
    weak: Entry[];
    stamp: StampState | null;
    error: string;
    onadd: () => void;
    onopen: (entry: Entry) => void;
  } = $props();

  const mod = typeof navigator !== 'undefined' && /Mac/.test(navigator.platform) ? '⌘' : 'Ctrl';
  const empty = $derived(total === 0);
</script>

<CardShell title={empty ? 'An empty drawer' : `${total} ${total === 1 ? 'login' : 'logins'} filed`} {stamp} {error}>
  {#snippet sub()}
    {empty ? 'Nothing filed yet.' : 'Open a card from the drawer, or search for one.'}
  {/snippet}

  {#if empty}
    <p class="mt-5 max-w-[48ch] text-sm leading-relaxed">
      Add your first login. It is encrypted with AES-256-GCM and saved only on this device.
    </p>
    <Button variant="primary" class="mt-5" onclick={onadd}>
      <Icon name="plus" />
      Add a login
    </Button>
  {:else if weak.length > 0}
    <section class="mt-5" aria-labelledby="weak-heading">
      <h2 id="weak-heading" class="text-base font-bold">
        {weak.length} weak {weak.length === 1 ? 'password' : 'passwords'}
      </h2>
      <p class="mt-1 text-sm text-muted">Short or simple. Replace these when you next sign in.</p>
      <ul class="mt-3 border-t border-hair">
        {#each weak.slice(0, 5) as entry (entry.id)}
          <li class="border-b border-hair">
            <button
              type="button"
              onclick={() => onopen(entry)}
              class="flex h-11 w-full items-center gap-3 rounded-md px-2 text-left hover:bg-select"
            >
              <span class="min-w-0 flex-1 truncate text-sm font-semibold">{entry.site}</span>
              <span class="min-w-0 truncate font-type text-[13px] text-muted max-sm:hidden">{entry.username}</span>
              <Stamp text="Weak" tone="weak" small />
            </button>
          </li>
        {/each}
      </ul>
      {#if weak.length > 5}
        <p class="mt-2 text-sm text-muted">And {weak.length - 5} more, marked in the drawer.</p>
      {/if}
    </section>
  {/if}

  <section class="mt-7" aria-labelledby="keys-heading">
    <h2 id="keys-heading" class="text-base font-bold">Keyboard</h2>
    <dl class="mt-3 grid grid-cols-[auto_1fr] items-center gap-x-4 gap-y-2.5 text-sm">
      <dt><kbd>/</kbd></dt>
      <dd class="text-muted">Search the drawer</dd>
      <dt class="flex gap-1">
        <kbd aria-label="Up arrow"><Icon name="arrow-up" class="size-3" /></kbd>
        <kbd aria-label="Down arrow"><Icon name="arrow-down" class="size-3" /></kbd>
      </dt>
      <dd class="text-muted">Move between logins while searching</dd>
      <dt><kbd>Enter</kbd></dt>
      <dd class="text-muted">Copy the selected password</dd>
      <dt class="flex gap-1"><kbd>{mod}</kbd><kbd>N</kbd></dt>
      <dd class="text-muted">New login</dd>
      <dt><kbd>Esc</kbd></dt>
      <dd class="text-muted">Close a card</dd>
    </dl>
  </section>
</CardShell>
