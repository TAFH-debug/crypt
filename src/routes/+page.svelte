<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { AUTO_LOCK_MS, CLIPBOARD_CLEAR_MS, filterEntries, strength, type Entry } from '$lib/password';
  import Button from './components/button.svelte';
  import Icon from './components/icon.svelte';
  import Input from './components/input.svelte';
  import LockScreen from './components/lock-screen.svelte';
  import PasswordList from './components/password-list.svelte';
  import AddPasswordModal from './components/add-password-modal.svelte';
  import ViewPasswordModal from './components/view-password-modal.svelte';
  import SettingsModal from './components/settings-modal.svelte';
  import Toast from './components/toast.svelte';

  // Kept out of reactive state: only used for saves, never rendered.
  let masterPassword = '';

  let unlocked = $state(false);
  let lockError = $state('');
  let entries: Entry[] = $state([]);
  let query = $state('');
  let selected: Entry | null = $state(null);
  let addOpen = $state(false);
  let settingsOpen = $state(false);
  let toast: string | null = $state(null);
  let toastTimer: ReturnType<typeof setTimeout> | undefined;

  const visible = $derived(filterEntries(entries, query));
  const weakCount = $derived(entries.filter((e) => strength(e.password).score < 2).length);
  const modalOpen = $derived(addOpen || settingsOpen || selected !== null);

  function greeting() {
    const h = new Date().getHours();
    return h < 12 ? 'Good morning' : h < 18 ? 'Good afternoon' : 'Good evening';
  }

  function notify(message: string) {
    toast = message;
    clearTimeout(toastTimer);
    toastTimer = setTimeout(() => (toast = null), 2000);
  }

  async function copy(text: string): Promise<boolean> {
    try {
      await navigator.clipboard.writeText(text);
      notify('Copied · clears in 30s');
      setTimeout(clearClipboardIf, CLIPBOARD_CLEAR_MS, text);
      return true;
    } catch {
      notify('Copy failed');
      return false;
    }
  }

  // Only wipes the clipboard if it still holds what we copied.
  async function clearClipboardIf(text: string) {
    try {
      if ((await navigator.clipboard.readText()) === text) await navigator.clipboard.writeText('');
    } catch {
      // Clipboard access denied: leave it alone.
    }
  }

  async function persist(next: Entry[], password = masterPassword): Promise<boolean> {
    try {
      await invoke('save_store', { store: next, password });
      return true;
    } catch {
      notify('Could not save');
      return false;
    }
  }

  async function unlock(password: string) {
    try {
      const res = await invoke<Entry[] | string>('get_store', { password });
      if (typeof res === 'string') {
        lockError = 'Wrong master password';
        return;
      }
      masterPassword = password;
      entries = res;
      lockError = '';
      unlocked = true;
    } catch {
      lockError = 'Could not open vault';
    }
  }

  function lock() {
    masterPassword = '';
    entries = [];
    query = '';
    selected = null;
    addOpen = false;
    settingsOpen = false;
    unlocked = false;
  }

  async function addEntry(entry: Entry) {
    const next = [...entries, entry];
    if (!(await persist(next))) return;
    entries = next;
    addOpen = false;
    notify('Saved');
  }

  async function removeEntry(id: number) {
    const next = entries.filter((e) => e.id !== id);
    if (!(await persist(next))) return;
    entries = next;
    selected = null;
    notify('Deleted');
  }

  async function changeMasterPassword(password: string) {
    if (!(await persist(entries, password))) return;
    masterPassword = password;
    settingsOpen = false;
    notify('Master password updated');
  }

  async function deleteVault() {
    try {
      await invoke('delete_database');
      lock();
    } catch {
      notify('Could not delete');
    }
  }

  // Lock after a period without input.
  $effect(() => {
    if (!unlocked) return;
    let timer: ReturnType<typeof setTimeout> | undefined;
    const autoLock = () => {
      lock();
      notify('Locked after inactivity');
    };
    const reset = () => {
      clearTimeout(timer);
      timer = setTimeout(autoLock, AUTO_LOCK_MS);
    };
    const events = ['pointerdown', 'keydown'] as const;
    events.forEach((ev) => window.addEventListener(ev, reset));
    reset();
    return () => {
      clearTimeout(timer);
      events.forEach((ev) => window.removeEventListener(ev, reset));
    };
  });

  function onKeydown(e: KeyboardEvent) {
    if (!unlocked) return;
    if ((e.metaKey || e.ctrlKey) && e.key.toLowerCase() === 'n') {
      e.preventDefault();
      addOpen = true;
      return;
    }
    const typing = e.target instanceof HTMLInputElement || e.target instanceof HTMLTextAreaElement;
    if (e.key === '/' && !typing && !modalOpen) {
      e.preventDefault();
      document.querySelector<HTMLInputElement>('input[type="search"]')?.focus();
    }
  }
</script>

<svelte:window onkeydown={onKeydown} />

{#if !unlocked}
  <LockScreen error={lockError} onunlock={unlock} />
{:else}
  <div class="pointer-events-none fixed inset-0 -z-10 overflow-hidden">
    <div class="animate-drift absolute -top-32 -right-24 h-96 w-96 rounded-full bg-accent-2/15 blur-3xl"></div>
    <div class="animate-drift absolute bottom-0 -left-24 h-80 w-80 rounded-full bg-accent-1/15 blur-3xl [animation-delay:-7s]"></div>
  </div>

  <main class="mx-auto max-w-2xl px-5 py-10">
    <header class="mb-8 flex items-start justify-between">
      <div>
        <p class="text-xs font-semibold tracking-wide text-zinc-400 uppercase">{greeting()}</p>
        <h1 class="text-3xl font-extrabold tracking-tight">Your vault</h1>
        {#if entries.length > 0}
          <p class="mt-2 flex items-center gap-2 text-sm text-zinc-500">
            <span>{entries.length} saved</span>
            {#if weakCount > 0}
              <span class="rounded-full bg-rose-500/10 px-2 py-0.5 text-xs font-semibold text-rose-500">{weakCount} weak</span>
            {/if}
          </p>
        {/if}
      </div>
      <div class="flex gap-1">
        <Button variant="ghost" class="p-2" title="Lock" aria-label="Lock" onclick={lock}>
          <Icon name="lock" />
        </Button>
        <Button variant="ghost" class="p-2" title="Settings" aria-label="Settings" onclick={() => (settingsOpen = true)}>
          <Icon name="settings" />
        </Button>
      </div>
    </header>

    <div class="mb-6 flex gap-2">
      <div class="relative flex-1">
        <Icon name="search" class="pointer-events-none absolute top-1/2 left-3.5 h-4 w-4 -translate-y-1/2 text-zinc-400" />
        <Input type="search" placeholder="Search  ·  press /" aria-label="Search passwords" class="pl-10" bind:value={query} />
      </div>
      <Button variant="primary" title="New password (Ctrl+N)" onclick={() => (addOpen = true)}>
        <Icon name="plus" />
        Add
      </Button>
    </div>

    <PasswordList
      entries={visible}
      hasQuery={query.trim() !== ''}
      onopen={(entry) => (selected = entry)}
      oncopy={copy}
      onadd={() => (addOpen = true)}
    />
  </main>
{/if}

{#if addOpen}
  <AddPasswordModal onsave={addEntry} onclose={() => (addOpen = false)} />
{/if}

{#if selected}
  <ViewPasswordModal entry={selected} oncopy={copy} ondelete={removeEntry} onclose={() => (selected = null)} />
{/if}

{#if settingsOpen}
  <SettingsModal onchangepassword={changeMasterPassword} ondelete={deleteVault} onclose={() => (settingsOpen = false)} />
{/if}

{#if toast}
  <Toast message={toast} />
{/if}
