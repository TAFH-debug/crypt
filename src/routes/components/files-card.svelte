<script lang="ts">
  import { formatBytes, formatDate, plural, type Entry, type StoredFile } from '$lib/password';
  import Button from './button.svelte';
  import CardShell from './card-shell.svelte';
  import Icon from './icon.svelte';
  import type { StampState } from './stamp.svelte';

  let {
    files,
    entries,
    stamp,
    error,
    onadd,
    onsave,
    onlink,
    onremove,
    onclose
  }: {
    files: StoredFile[];
    entries: Entry[];
    stamp: StampState | null;
    error: string;
    onadd: () => Promise<void>;
    onsave: (file: StoredFile) => Promise<void>;
    onlink: (id: number, entryId: number | null) => Promise<void>;
    onremove: (id: number) => Promise<void>;
    onclose: () => void;
  } = $props();

  let adding = $state(false);
  let confirmId = $state<number | null>(null);

  const sorted = $derived([...files].sort((a, b) => a.name.localeCompare(b.name)));
  const logins = $derived([...entries].sort((a, b) => a.site.localeCompare(b.site)));

  async function add() {
    adding = true;
    await onadd();
    adding = false;
  }

  const linkValue = (file?: StoredFile) => (file?.entryId === undefined ? '' : String(file.entryId));

  async function link(file: StoredFile, select: HTMLSelectElement) {
    await onlink(file.id, select.value === '' ? null : Number(select.value));
    // On failure the list is unchanged; put the select back to match it.
    select.value = linkValue(files.find((f) => f.id === file.id));
  }
</script>

<CardShell title={files.length ? `${plural(files.length, 'file')} filed` : 'No files yet'} {stamp} {error} onback={onclose}>
  {#snippet sub()}Encrypted inside this vault, up to 100 MB each. Link one to a login to see it on that card.{/snippet}

  <Button variant="primary" class="mt-5" disabled={adding} onclick={add}>
    <Icon name="plus" />
    {adding ? 'Encrypting…' : 'Add file'}
  </Button>

  {#if sorted.length > 0}
    <ul class="mt-5 border-t border-hair">
      {#each sorted as file (file.id)}
        <li class="border-b border-hair py-3">
          <div class="flex items-center gap-3">
            <Icon name="file" class="size-5 shrink-0 text-muted" />
            <div class="min-w-0 flex-1">
              <p class="truncate text-sm font-semibold" title={file.name}>{file.name}</p>
              <p class="text-xs text-muted tabular-nums">{formatBytes(file.size)}&ensp;·&ensp;added {formatDate(file.added)}</p>
            </div>
            <button
              type="button"
              title="Save a decrypted copy"
              aria-label={`Save a copy of ${file.name}`}
              onclick={() => onsave(file)}
              class="grid size-9 shrink-0 place-items-center rounded-lg text-muted transition-colors duration-150 hover:bg-select hover:text-active"
            >
              <Icon name="download" class="size-[18px]" />
            </button>
          </div>

          {#if confirmId === file.id}
            <div class="mt-2 flex flex-wrap items-center gap-2 pl-8">
              <p class="mr-auto text-sm">Remove this file? Copies you saved elsewhere stay.</p>
              <Button variant="ghost" onclick={() => (confirmId = null)}>Keep it</Button>
              <Button variant="danger" onclick={() => onremove(file.id)}>Remove file</Button>
            </div>
          {:else}
            <div class="mt-2 flex items-center gap-2 pl-8 max-sm:flex-col max-sm:items-stretch">
              <select
                aria-label={`Login linked to ${file.name}`}
                value={linkValue(file)}
                onchange={(e) => link(file, e.currentTarget)}
                class="h-9 min-w-0 flex-1 rounded-lg border border-line bg-card px-2 text-sm text-ink transition-colors duration-150 outline-none hover:border-active/50 focus:border-active focus:ring-3 focus:ring-active/20"
              >
                <option value="">Not linked to a login</option>
                {#each logins as entry (entry.id)}
                  <option value={String(entry.id)}>{entry.site} · {entry.username}</option>
                {/each}
              </select>
              <Button variant="ghost" class="h-9" onclick={() => (confirmId = file.id)}>Remove</Button>
            </div>
          {/if}
        </li>
      {/each}
    </ul>
  {/if}
</CardShell>
