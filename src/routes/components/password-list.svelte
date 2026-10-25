<script lang="ts">
  import { fly } from 'svelte/transition';
  import { hueFor, type Entry } from '$lib/password';
  import Button from './button.svelte';
  import CopyButton from './copy-button.svelte';
  import Icon from './icon.svelte';

  let {
    entries,
    hasQuery,
    onopen,
    oncopy,
    onadd
  }: {
    entries: Entry[];
    hasQuery: boolean;
    onopen: (entry: Entry) => void;
    oncopy: (text: string) => Promise<boolean>;
    onadd: () => void;
  } = $props();
</script>

{#if entries.length === 0}
  <div class="flex flex-col items-center py-20 text-center" in:fly={{ y: 8, duration: 250 }}>
    {#if hasQuery}
      <p class="text-sm text-zinc-500">Nothing matches that search.</p>
    {:else}
      <div class="mb-4 flex h-16 w-16 items-center justify-center rounded-3xl bg-linear-to-br from-accent-1/15 to-accent-2/15 text-accent-1">
        <Icon name="sparkles" class="h-7 w-7" />
      </div>
      <p class="text-lg font-bold">Your vault is empty</p>
      <p class="mt-1 text-sm text-zinc-500">Save your first password to get started.</p>
      <Button variant="primary" class="mt-6" onclick={onadd}>
        <Icon name="plus" />
        Add password
      </Button>
    {/if}
  </div>
{:else}
  <ul class="space-y-2">
    {#each entries as entry, i (entry.id)}
      {@const hue = hueFor(entry.site)}
      <li
        class="group flex items-center gap-2 rounded-2xl bg-white p-2 pr-2 shadow-sm ring-1 ring-zinc-200/70 transition hover:-translate-y-0.5 hover:shadow-md dark:bg-zinc-900 dark:ring-zinc-800"
        in:fly={{ y: 10, duration: 250, delay: Math.min(i, 12) * 35 }}
      >
        <button
          type="button"
          onclick={() => onopen(entry)}
          class="flex min-w-0 flex-1 items-center gap-3 rounded-xl px-1.5 py-1 text-left"
        >
          <span
            class="flex h-10 w-10 shrink-0 items-center justify-center rounded-xl text-sm font-bold text-white uppercase shadow-inner"
            style={`background: linear-gradient(135deg, hsl(${hue} 85% 62%), hsl(${(hue + 45) % 360} 85% 55%))`}
          >
            {entry.site.charAt(0)}
          </span>
          <span class="min-w-0">
            <span class="block truncate text-sm font-semibold">{entry.site}</span>
            <span class="block truncate text-xs text-zinc-500">{entry.username}</span>
          </span>
        </button>
        <CopyButton text={entry.password} label="Copy password" oncopy={oncopy} />
      </li>
    {/each}
  </ul>
{/if}
