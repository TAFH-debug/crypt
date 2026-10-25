<script lang="ts">
  import { hueFor, REVEAL_MS, strength, type Entry } from '$lib/password';
  import Button from './button.svelte';
  import CopyButton from './copy-button.svelte';
  import Icon from './icon.svelte';
  import Modal from './modal.svelte';

  let {
    entry,
    oncopy,
    ondelete,
    onclose
  }: {
    entry: Entry;
    oncopy: (text: string) => Promise<boolean>;
    ondelete: (id: number) => Promise<void>;
    onclose: () => void;
  } = $props();

  let showPassword = $state(false);
  let confirming = $state(false);

  const hue = $derived(hueFor(entry.site));
  const health = $derived(strength(entry.password));

  // Revealed passwords hide themselves again after a short time.
  $effect(() => {
    if (!showPassword) return;
    const timer = setTimeout(() => (showPassword = false), REVEAL_MS);
    return () => clearTimeout(timer);
  });
</script>

{#snippet row(label: string, value: string, copyLabel: string)}
  <div class="flex items-center justify-between gap-3 py-3.5">
    <div class="min-w-0">
      <p class="text-[11px] font-semibold tracking-wide text-zinc-400 uppercase">{label}</p>
      <p class="truncate text-sm font-medium">{value}</p>
    </div>
    <CopyButton text={value} label={copyLabel} {oncopy} />
  </div>
{/snippet}

<Modal title="" {onclose}>
  <div class="mb-5 flex items-center gap-3">
    <span
      class="flex h-12 w-12 shrink-0 items-center justify-center rounded-2xl text-lg font-bold text-white uppercase"
      style={`background: linear-gradient(135deg, hsl(${hue} 85% 62%), hsl(${(hue + 45) % 360} 85% 55%))`}
    >
      {entry.site.charAt(0)}
    </span>
    <div class="min-w-0">
      <p class="truncate text-lg font-bold">{entry.site}</p>
      {#if health.score >= 0}
        <p class="text-xs font-medium {health.score >= 3 ? 'text-emerald-500' : health.score === 2 ? 'text-amber-500' : 'text-rose-500'}">
          {health.label} password
        </p>
      {/if}
    </div>
  </div>

  <div class="divide-y divide-zinc-100 dark:divide-zinc-800">
    {@render row('Username', entry.username, 'Copy username')}

    <div class="flex items-center justify-between gap-3 py-3.5">
      <div class="min-w-0">
        <p class="text-[11px] font-semibold tracking-wide text-zinc-400 uppercase">Password</p>
        <p class="truncate font-mono text-sm font-medium">{showPassword ? entry.password : '••••••••••••'}</p>
      </div>
      <div class="flex shrink-0 items-center">
        <Button variant="ghost" class="p-2" aria-label={showPassword ? 'Hide password' : 'Show password'} onclick={() => (showPassword = !showPassword)}>
          <Icon name={showPassword ? 'eye-off' : 'eye'} />
        </Button>
        <CopyButton text={entry.password} label="Copy password" {oncopy} />
      </div>
    </div>

    {#if entry.notes}
      <div class="py-3.5">
        <p class="text-[11px] font-semibold tracking-wide text-zinc-400 uppercase">Notes</p>
        <p class="text-sm whitespace-pre-wrap">{entry.notes}</p>
      </div>
    {/if}

    {#if entry.date}
      <div class="py-3.5">
        <p class="text-[11px] font-semibold tracking-wide text-zinc-400 uppercase">Added</p>
        <p class="text-sm">{new Date(entry.date).toLocaleDateString()}</p>
      </div>
    {/if}
  </div>

  <div class="mt-6 flex items-center justify-between">
    {#if confirming}
      <div class="flex gap-2">
        <Button variant="danger" onclick={() => ondelete(entry.id)}>Confirm delete</Button>
        <Button variant="ghost" onclick={() => (confirming = false)}>Cancel</Button>
      </div>
    {:else}
      <Button variant="ghost" class="text-rose-500 hover:text-rose-600" onclick={() => (confirming = true)}>Delete</Button>
    {/if}
    <Button variant="secondary" onclick={onclose}>Done</Button>
  </div>
</Modal>
