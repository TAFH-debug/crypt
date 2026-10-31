<script lang="ts">
  import { fileLetter, formatDate, REVEAL_MS, type Entry } from '$lib/password';
  import Button from './button.svelte';
  import CardShell from './card-shell.svelte';
  import CopyButton from './copy-button.svelte';
  import Icon from './icon.svelte';
  import type { StampState } from './stamp.svelte';
  import StrengthMeter from './strength-meter.svelte';

  let {
    entry,
    stamp,
    stampNote,
    error,
    tabPos,
    oncopy,
    ondelete,
    onback
  }: {
    entry: Entry;
    /** Which third of the drawer this login's guide tab sits in. */
    tabPos: number;
    stamp: StampState | null;
    stampNote: string;
    error: string;
    oncopy: (field: 'username' | 'password') => Promise<boolean>;
    ondelete: (id: number) => Promise<void>;
    onback: () => void;
  } = $props();

  let showPassword = $state(false);
  let confirming = $state(false);

  // Revealed passwords hide themselves again after a short time.
  $effect(() => {
    if (!showPassword) return;
    const timer = setTimeout(() => (showPassword = false), REVEAL_MS);
    return () => clearTimeout(timer);
  });
</script>

<CardShell title={entry.site} {stamp} {stampNote} {error} {onback} tab={{ letter: fileLetter(entry.site), pos: tabPos }}>
  {#snippet sub()}
    Filed under {fileLetter(entry.site)}{#if entry.date}&ensp;·&ensp;added {formatDate(entry.date)}{/if}
  {/snippet}

  <dl>
    <div class="grid grid-cols-[112px_1fr_auto] items-center gap-3 border-b border-hair py-3 max-sm:grid-cols-[1fr_auto]">
      <dt class="text-[11px] font-semibold tracking-[0.08em] text-muted uppercase max-sm:col-span-2">Username</dt>
      <dd class="min-w-0 truncate font-type text-[17px]">{entry.username}</dd>
      <dd><CopyButton label="Copy username" oncopy={() => oncopy('username')} /></dd>
    </div>

    <div class="grid grid-cols-[112px_1fr_auto] items-center gap-3 border-b border-hair py-3 max-sm:grid-cols-[1fr_auto]">
      <dt class="text-[11px] font-semibold tracking-[0.08em] text-muted uppercase max-sm:col-span-2">Password</dt>
      <dd class="min-w-0 space-y-2">
        <p class="truncate font-type text-[17px]" aria-live="polite">
          {#if showPassword}
            {entry.password}
          {:else}
            <span aria-label="Hidden password">••••••••••••••••</span>
          {/if}
        </p>
        <StrengthMeter password={entry.password} />
      </dd>
      <dd class="flex items-center">
        <button
          type="button"
          title={showPassword ? 'Hide password' : 'Show password for 10 seconds'}
          aria-label={showPassword ? 'Hide password' : 'Show password'}
          aria-pressed={showPassword}
          onclick={() => (showPassword = !showPassword)}
          class="grid size-9 place-items-center rounded-lg text-muted transition-colors duration-150 hover:bg-select hover:text-active"
        >
          <Icon name={showPassword ? 'eye-off' : 'eye'} class="size-[18px]" />
        </button>
        <CopyButton label="Copy password" oncopy={() => oncopy('password')} />
      </dd>
    </div>

    {#if entry.notes}
      <div class="grid grid-cols-[112px_1fr] gap-3 border-b border-hair py-4 max-sm:grid-cols-1">
        <dt class="pt-0.5 text-[11px] font-semibold tracking-[0.08em] text-muted uppercase">Notes</dt>
        <dd class="max-w-[60ch] text-sm leading-relaxed whitespace-pre-wrap [overflow-wrap:anywhere]">{entry.notes}</dd>
      </div>
    {/if}
  </dl>

  <div class="mt-6 flex min-h-10 flex-wrap items-center gap-3">
    {#if confirming}
      <p class="mr-auto text-sm">Delete this login? It can't be undone.</p>
      <Button variant="ghost" onclick={() => (confirming = false)}>Keep it</Button>
      <Button variant="danger" onclick={() => ondelete(entry.id)}>Delete login</Button>
    {:else}
      <Button variant="ghost" class="-ml-3" onclick={() => (confirming = true)}>Delete login</Button>
    {/if}
  </div>
</CardShell>
