<script lang="ts">
  import { MIN_PASSWORD_LENGTH } from '$lib/password';
  import Button from './button.svelte';
  import CardShell from './card-shell.svelte';
  import Input from './input.svelte';
  import type { StampState } from './stamp.svelte';

  let {
    recoveryQuestion,
    stamp,
    error,
    onchangepassword,
    onsetrecovery,
    ondelete,
    onclose
  }: {
    recoveryQuestion: string | null;
    stamp: StampState | null;
    error: string;
    onchangepassword: (password: string) => Promise<boolean>;
    onsetrecovery: (question: string, answer: string) => Promise<boolean>;
    ondelete: () => Promise<void>;
    onclose: () => void;
  } = $props();

  let newPassword = $state('');
  let passwordError = $state('');

  let question = $state('');
  let answer = $state('');

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

  <section class="pt-6" aria-labelledby="set-delete">
    {@render heading('set-delete', 'Delete vault')}
    <p class="mt-1 max-w-[52ch] text-sm text-muted">Permanently erases every saved login on this device. Type DELETE to confirm.</p>
    <div class="mt-4 flex gap-2 max-sm:flex-col">
      <Input aria-label="Type DELETE to confirm" placeholder="DELETE" autocomplete="off" spellcheck="false" bind:value={confirmText} />
      <Button variant="danger" disabled={!canDelete} onclick={() => ondelete()}>Delete vault</Button>
    </div>
  </section>
</CardShell>
