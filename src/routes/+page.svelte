<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { open, save } from '@tauri-apps/plugin-dialog';
  import {
    AUTO_LOCK_MS,
    CLIPBOARD_CLEAR_MS,
    fileLetter,
    fileLetters,
    filterEntries,
    strength,
    type Entry,
    type ImportSummary,
    type StoredFile,
    type VaultStatus
  } from '$lib/password';
  import Drawer from './components/drawer.svelte';
  import EntryCard from './components/entry-card.svelte';
  import FilesCard from './components/files-card.svelte';
  import LockScreen from './components/lock-screen.svelte';
  import NewCard from './components/new-card.svelte';
  import OverviewCard from './components/overview-card.svelte';
  import Rail from './components/rail.svelte';
  import SettingsCard from './components/settings-card.svelte';
  import type { StampState, Tone } from './components/stamp.svelte';

  type Pane = 'entry' | 'new' | 'files' | 'settings';
  type Clip = { entryId: number; field: 'username' | 'password'; until: number };

  // The master password never lives in the frontend. Rust keeps the session key.
  let unlocked = $state(false);
  let status: VaultStatus = $state({ exists: false, recoveryQuestion: null });
  let lockError = $state('');
  let lockNotice = $state('');
  let entries: Entry[] = $state([]);
  let files: StoredFile[] = $state([]);
  let query = $state('');
  let selectedId = $state<number | null>(null);
  let pane = $state<Pane>('entry');
  let stamp = $state<StampState | null>(null);
  let error = $state('');
  let clip = $state<Clip | null>(null);
  let now = $state(Date.now());

  const visible = $derived(filterEntries(entries, query));
  const weak = $derived(filterEntries(entries, '').filter((e) => strength(e.password).score < 2));
  const selected = $derived(entries.find((e) => e.id === selectedId) ?? null);
  const selectedFiles = $derived(selected ? files.filter((f) => f.entryId === selected.id) : []);
  // The open card's guide tab sits where its letter's tab sits in the drawer.
  const tabPos = $derived.by(() => {
    if (!selected) return 0;
    const letter = fileLetter(selected.site);
    const shown = fileLetters(visible);
    const order = shown.includes(letter) ? shown : fileLetters(filterEntries(entries, ''));
    return Math.max(0, order.indexOf(letter)) % 3;
  });
  // On narrow windows the pane covers the drawer only while it has something to show.
  const paneActive = $derived(pane !== 'entry' || selected !== null);

  const clipSeconds = $derived(clip ? Math.max(0, Math.ceil((clip.until - now) / 1000)) : 0);
  const cardStamp = $derived<StampState | null>(
    clip && pane === 'entry' && clip.entryId === selectedId
      ? { id: clip.until, text: `Copied ${clip.field}`, tone: 'ink' }
      : stamp
  );
  const cardStampNote = $derived(clip && cardStamp?.id === clip.until ? `clipboard clears in ${clipSeconds}s` : '');

  onMount(refreshStatus);

  function mark(text: string, tone: Tone = 'ink') {
    error = '';
    stamp = { id: Date.now(), text, tone };
  }

  function fail(e: unknown) {
    stamp = null;
    error = String(e);
  }

  function show(next: Pane, id: number | null = selectedId) {
    pane = next;
    selectedId = id;
    stamp = null;
    error = '';
  }

  async function refreshStatus() {
    try {
      status = await invoke<VaultStatus>('vault_status');
    } catch (e) {
      lockError = String(e);
    }
  }

  async function copy(entry: Entry, field: 'username' | 'password'): Promise<boolean> {
    const text = field === 'password' ? entry.password : entry.username;
    try {
      await navigator.clipboard.writeText(text);
    } catch {
      if (selectedId !== entry.id) show('entry', entry.id);
      fail('Could not reach the clipboard. Try again.');
      return false;
    }
    if (pane !== 'entry' || selectedId !== entry.id) show('entry', entry.id);
    clip = { entryId: entry.id, field, until: Date.now() + CLIPBOARD_CLEAR_MS };
    now = Date.now();
    setTimeout(clearClipboardIf, CLIPBOARD_CLEAR_MS, text);
    return true;
  }

  // Only wipes the clipboard if it still holds what we copied.
  async function clearClipboardIf(text: string) {
    try {
      if ((await navigator.clipboard.readText()) === text) await navigator.clipboard.writeText('');
    } catch {
      // Clipboard access denied: leave it alone.
    }
  }

  // Tick the clipboard countdown while a copy is live.
  $effect(() => {
    if (!clip) return;
    const until = clip.until;
    const timer = setInterval(() => {
      now = Date.now();
      if (now >= until) clip = null;
    }, 1000);
    return () => clearInterval(timer);
  });

  async function unlock(password: string) {
    lockError = '';
    try {
      entries = await invoke<Entry[]>('unlock', { password });
      lockError = '';
      lockNotice = '';
      unlocked = true;
      await Promise.all([refreshStatus(), loadFiles()]);
    } catch (e) {
      lockError = String(e);
    }
  }

  async function recover(answer: string, password: string) {
    lockError = '';
    try {
      entries = await invoke<Entry[]>('recover', { answer, password });
      lockError = '';
      lockNotice = '';
      unlocked = true;
      mark('Password reset');
      await Promise.all([refreshStatus(), loadFiles()]);
    } catch (e) {
      lockError = String(e);
    }
  }

  async function loadFiles() {
    try {
      files = await invoke<StoredFile[]>('list_files');
    } catch (e) {
      fail(e);
    }
  }

  function lock(notice = '') {
    entries = [];
    files = [];
    query = '';
    clip = null;
    show('entry', null);
    lockNotice = notice;
    unlocked = false;
    invoke('lock').finally(refreshStatus);
  }

  async function addEntry(entry: Entry) {
    const next = [...entries, entry];
    try {
      await invoke('save_store', { store: next });
      entries = next;
      query = '';
      show('entry', entry.id);
      mark('Filed');
    } catch (e) {
      fail(e);
    }
  }

  async function removeEntry(id: number) {
    const next = entries.filter((e) => e.id !== id);
    try {
      await invoke('save_store', { store: next });
      entries = next;
      // Rust unlinks the login's files; mirror it.
      files = files.map((f) => (f.entryId === id ? { ...f, entryId: undefined } : f));
      if (clip?.entryId === id) clip = null;
      show('entry', null);
      mark('Removed');
    } catch (e) {
      fail(e);
    }
  }

  async function changeMasterPassword(password: string): Promise<boolean> {
    try {
      await invoke('change_master_password', { password });
      mark('Updated');
      return true;
    } catch (e) {
      fail(e);
      return false;
    }
  }

  async function setRecovery(question: string, answer: string): Promise<boolean> {
    try {
      await invoke('set_recovery', { question, answer });
      mark('Saved');
      await refreshStatus();
      return true;
    } catch (e) {
      fail(e);
      return false;
    }
  }

  async function addFile(entryId: number | null = null) {
    const path = await open({ title: 'Add a file to the vault', multiple: false, directory: false });
    if (typeof path !== 'string') return;
    try {
      files = await invoke<StoredFile[]>('add_file', { path, entryId });
      mark('Filed');
    } catch (e) {
      fail(e);
    }
  }

  async function saveFileCopy(file: StoredFile) {
    const path = await save({ title: 'Save a decrypted copy', defaultPath: file.name });
    if (!path) return;
    try {
      await invoke('save_file_copy', { id: file.id, path });
      mark('Saved');
    } catch (e) {
      fail(e);
    }
  }

  async function linkFile(id: number, entryId: number | null) {
    try {
      files = await invoke<StoredFile[]>('link_file', { id, entryId });
      mark(entryId === null ? 'Unlinked' : 'Linked');
    } catch (e) {
      fail(e);
    }
  }

  async function removeFile(id: number) {
    try {
      files = await invoke<StoredFile[]>('remove_file', { id });
      mark('Removed');
    } catch (e) {
      fail(e);
    }
  }

  async function exportVault() {
    const date = new Date().toISOString().slice(0, 10);
    const path = await save({
      title: 'Export an encrypted backup',
      defaultPath: `crypt-backup-${date}.cryptbak`,
      filters: [{ name: 'Crypt backup', extensions: ['cryptbak'] }]
    });
    if (!path) return;
    try {
      await invoke('export_vault', { path });
      mark('Exported');
    } catch (e) {
      fail(e);
    }
  }

  async function importVault(path: string, password: string): Promise<ImportSummary | null> {
    try {
      const summary = await invoke<ImportSummary>('import_vault', { path, password });
      entries = summary.entries;
      files = summary.files;
      mark('Imported');
      return summary;
    } catch (e) {
      fail(e);
      return null;
    }
  }

  async function deleteVault() {
    try {
      await invoke('delete_database');
      lock('Deleted');
    } catch (e) {
      fail(e);
    }
  }

  function move(delta: number) {
    if (visible.length === 0) return;
    const at = visible.findIndex((e) => e.id === selectedId);
    const next = at === -1 ? (delta > 0 ? 0 : visible.length - 1) : Math.min(visible.length - 1, Math.max(0, at + delta));
    show('entry', visible[next].id);
  }

  function copySelected() {
    const entry = visible.find((e) => e.id === selectedId) ?? (visible.length === 1 ? visible[0] : null);
    if (entry) copy(entry, 'password');
  }

  // Lock after a period without input.
  $effect(() => {
    if (!unlocked) return;
    let timer: ReturnType<typeof setTimeout> | undefined;
    const reset = () => {
      clearTimeout(timer);
      timer = setTimeout(() => lock('Auto-locked'), AUTO_LOCK_MS);
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
      show('new');
      return;
    }
    if (e.key === 'Escape' && !e.defaultPrevented) {
      if (pane !== 'entry') show('entry');
      else if (selectedId !== null) show('entry', null);
      return;
    }
    const typing = e.target instanceof HTMLInputElement || e.target instanceof HTMLTextAreaElement;
    if (e.key === '/' && !typing) {
      e.preventDefault();
      document.querySelector<HTMLInputElement>('input[type="search"]')?.focus();
    }
  }
</script>

<svelte:window onkeydown={onKeydown} />

{#if !unlocked}
  <LockScreen
    exists={status.exists}
    question={status.recoveryQuestion}
    error={lockError}
    notice={lockNotice}
    onunlock={unlock}
    onrecover={recover}
  />
{:else}
  <div class="grid h-screen grid-cols-[64px_minmax(300px,380px)_minmax(0,1fr)] overflow-hidden max-[760px]:grid-cols-[56px_minmax(0,1fr)]">
    <Rail {pane} onadd={() => show('new')} onfiles={() => show('files')} onsettings={() => show('settings')} onlock={() => lock()} />

    <Drawer
      entries={visible}
      total={entries.length}
      weakCount={weak.length}
      bind:query
      selectedId={pane === 'entry' ? selectedId : null}
      onselect={(entry) => show('entry', entry.id)}
      oncopy={(entry) => copy(entry, 'password')}
      onmove={move}
      oncopyselected={copySelected}
    />

    <main
      class={`min-h-0 overflow-y-auto bg-ground px-8 py-7 max-sm:px-3 max-sm:py-3 max-[760px]:fixed max-[760px]:inset-y-0 max-[760px]:right-0 max-[760px]:left-14 max-[760px]:z-20 ${paneActive ? '' : 'max-[760px]:hidden'}`}
    >
      {#if pane === 'new'}
        <NewCard {error} onsave={addEntry} oncancel={() => show('entry')} />
      {:else if pane === 'files'}
        <FilesCard
          {files}
          {entries}
          {stamp}
          {error}
          onadd={() => addFile()}
          onsave={saveFileCopy}
          onlink={linkFile}
          onremove={removeFile}
          onclose={() => show('entry')}
        />
      {:else if pane === 'settings'}
        <SettingsCard
          recoveryQuestion={status.recoveryQuestion}
          {stamp}
          {error}
          onchangepassword={changeMasterPassword}
          onsetrecovery={setRecovery}
          onexport={exportVault}
          onimport={importVault}
          ondelete={deleteVault}
          onclose={() => show('entry')}
        />
      {:else if selected}
        {#key selected.id}
          <EntryCard
            entry={selected}
            files={selectedFiles}
            stamp={cardStamp}
            stampNote={cardStampNote}
            {tabPos}
            {error}
            oncopy={(field) => copy(selected, field)}
            onattach={() => addFile(selected.id)}
            onsavefile={saveFileCopy}
            ondelete={removeEntry}
            onback={() => show('entry', null)}
          />
        {/key}
      {:else}
        <OverviewCard
          total={entries.length}
          {weak}
          {stamp}
          {error}
          onadd={() => show('new')}
          onopen={(entry) => show('entry', entry.id)}
        />
      {/if}
    </main>
  </div>
{/if}
