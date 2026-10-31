<script lang="ts">
  import { open } from '@tauri-apps/plugin-dialog';
  import { baseName, MIN_PASSWORD_LENGTH, plural, type ImportSummary } from '$lib/password';
  import Button from './button.svelte';
  import CardShell from './card-shell.svelte';
  import Icon from './icon.svelte';
  import Input from './input.svelte';
  import type { StampState } from './stamp.svelte';

  let {
    recoveryQuestion,
    stamp,
    error,
    onchangepassword,
    onsetrecovery,
    onexport,
    onimport,
    ondelete,
    onclose
  }: {
    recoveryQuestion: string | null;
    stamp: StampState | null;
    error: string;
    onchangepassword: (password: string) => Promise<boolean>;
    onsetrecovery: (question: string, answer: string) => Promise<boolean>;
    onexport: () => Promise<void>;
    onimport: (path: string, password: string) => Promise<ImportSummary | null>;
    ondelete: () => Promise<void>;
    onclose: () => void;
  } = $props();

  let newPassword = $state('');
  let passwordError = $state('');

  let question = $state('');
  let answer = $state('');

  let exporting = $state(false);

  let importPath = $state('');
  let importPassword = $state('');
  let importing = $state(false);
  let importResult = $state('');

  let confirmText = $state('');
  const canDelete = $derived(confirmText === 'DELETE');

  async function changePassword(e: SubmitEvent) {
    e.preventDefault();
    if (newPassword.length < MIN_PASSWORD_LENGTH) {
      passwordError = `Use at least ${MIN_PASSWORD_LENGTH} characters.`;
      return;
    }
    passwordError = '';
    if (await onchangepassword(newPassword)) newPassword = '';
  }

  async function saveRecovery(e: SubmitEvent) {
    e.preventDefault();
    if (await onsetrecovery(question, answer)) {
      question = '';
      answer = '';
    }
  }

  async function exportBackup() {
    exporting = true;
    await onexport();
    exporting = false;
  }

  async function pickBackup() {
    const path = await open({
      title: 'Choose a Crypt backup',
      multiple: false,
      directory: false,
      filters: [{ name: 'Crypt backup', extensions: ['cryptbak', 'json'] }]
    });
    if (typeof path !== 'string') return;
    importPath = path;
    importResult = '';
  }

  async function runImport(e: SubmitEvent) {
    e.preventDefault();
    if (!importPath || !importPassword) return;
    importing = true;
    const summary = await onimport(importPath, importPassword);
    importing = false;
    if (!summary) return;
    importPath = '';
    importPassword = '';
    importResult = describe(summary);
  }

  function describe(s: ImportSummary): string {
    const added = [s.addedEntries > 0 && plural(s.addedEntries, 'login'), s.addedFiles > 0 && plural(s.addedFiles, 'file')].filter(Boolean);
    const head = added.length ? `Added ${added.join(' and ')}.` : 'Nothing new to add.';
    return s.skipped > 0 ? `${head} Skipped ${s.skipped} already in this vault.` : head;
  }
</script>

{#snippet heading(id: string, text: string, tone = 'text-ink')}
  <h2 {id} class={`text-base font-bold ${tone}`}>{text}</h2>
{/snippet}

<CardShell title="Vault settings" {stamp} {error} onback={onclose}>
  {#snippet sub()}Changes apply to this device's vault only.{/snippet}

  <section class="border-b border-hair py-6" aria-labelledby="set-master">
    {@render heading('set-master', 'Master password')}
    <p class="mt-1 max-w-[52ch] text-sm text-muted">Rewraps the vault key with a new password. Your logins are not re-encrypted.</p>
    <form onsubmit={changePassword} class="mt-4 flex gap-2 max-sm:flex-col">
      <Input
        type="password"
        typed
        aria-label="New master password"
        placeholder="New master password"
        autocomplete="new-password"
        bind:value={newPassword}
      />
      <Button type="submit">Update password</Button>
    </form>
    {#if passwordError}
      <p role="alert" class="mt-2 text-sm font-semibold text-weak">{passwordError}</p>
    {/if}
  </section>

  <section class="border-b border-hair py-6" aria-labelledby="set-recovery">
    {@render heading('set-recovery', 'Recovery question')}
    <p class="mt-1 max-w-[52ch] text-sm text-muted">
      {#if recoveryQuestion}
        Current question: <span class="font-semibold text-ink">{recoveryQuestion}</span> Saving a new one replaces it.
      {:else}
        Lets you reset the master password from the lock screen if you forget it. Pick an answer only you know.
      {/if}
    </p>
    <form onsubmit={saveRecovery} class="mt-4 space-y-2">
      <Input aria-label="Recovery question" placeholder="Question, e.g. Name of your first street?" autocomplete="off" required bind:value={question} />
      <div class="flex gap-2 max-sm:flex-col">
        <Input aria-label="Recovery answer" typed placeholder="Answer" autocomplete="off" spellcheck="false" required bind:value={answer} />
        <Button type="submit">Save question</Button>
      </div>
    </form>
  </section>

  <section class="border-b border-hair py-6" aria-labelledby="set-protect">
    {@render heading('set-protect', 'How this vault is protected')}
    <ul class="mt-3 space-y-2 text-sm text-muted">
      <li>Logins are encrypted with <span class="font-semibold text-ink">AES-256-GCM</span>.</li>
      <li>Keys come from your password through <span class="font-semibold text-ink">Argon2id</span>.</li>
      <li>The vault file never leaves this device. Crypt has no account and no sync.</li>
    </ul>
  </section>

  <section class="border-b border-hair py-6" aria-labelledby="set-export">
    {@render heading('set-export', 'Export backup')}
    <p class="mt-1 max-w-[52ch] text-sm text-muted">
      Writes every login and file into one encrypted file. It opens with your current master password. The recovery question is left out.
    </p>
    <Button class="mt-4" disabled={exporting} onclick={exportBackup}>
      <Icon name="download" />
      {exporting ? 'Exporting…' : 'Export backup'}
    </Button>
  </section>

  <section class="border-b border-hair py-6" aria-labelledby="set-import">
    {@render heading('set-import', 'Import backup')}
    <p class="mt-1 max-w-[52ch] text-sm text-muted">
      Adds logins and files from a Crypt backup or another device's vault.json. Anything already in this vault is skipped.
    </p>
    <form onsubmit={runImport} class="mt-4 space-y-2">
      <div class="flex min-w-0 items-center gap-3">
        <Button onclick={pickBackup}>Choose file</Button>
        <span class={`min-w-0 truncate text-sm ${importPath ? 'font-semibold text-ink' : 'text-muted'}`} title={importPath}>
          {importPath ? baseName(importPath) : 'No file chosen'}
        </span>
      </div>
      {#if importPath}
        <div class="flex gap-2 max-sm:flex-col">
          <Input
            type="password"
            typed
            aria-label="Master password of the backup"
            placeholder="Master password of the backup"
            autocomplete="off"
            required
            bind:value={importPassword}
          />
          <Button type="submit" variant="primary" disabled={importing || !importPassword}>{importing ? 'Importing…' : 'Import'}</Button>
        </div>
      {/if}
    </form>
    {#if importResult}
      <p role="status" class="mt-2 text-sm text-muted">{importResult}</p>
    {/if}
  </section>

  <section class="pt-6" aria-labelledby="set-delete">
    {@render heading('set-delete', 'Delete vault')}
    <p class="mt-1 max-w-[52ch] text-sm text-muted">Permanently erases every saved login on this device. Type DELETE to confirm.</p>
    <div class="mt-4 flex gap-2 max-sm:flex-col">
      <Input aria-label="Type DELETE to confirm" placeholder="DELETE" autocomplete="off" spellcheck="false" bind:value={confirmText} />
      <Button variant="danger" disabled={!canDelete} onclick={() => ondelete()}>Delete vault</Button>
    </div>
  </section>
</CardShell>
