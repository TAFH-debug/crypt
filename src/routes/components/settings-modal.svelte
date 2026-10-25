<script lang="ts">
  import { MIN_PASSWORD_LENGTH } from '$lib/password';
  import Button from './button.svelte';
  import Input from './input.svelte';
  import Modal from './modal.svelte';

  let {
    recoveryQuestion,
    onchangepassword,
    onsetrecovery,
    ondelete,
    onclose
  }: {
    recoveryQuestion: string | null;
    onchangepassword: (password: string) => Promise<void>;
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
      passwordError = `Use at least ${MIN_PASSWORD_LENGTH} characters`;
      return;
    }
    passwordError = '';
    await onchangepassword(newPassword);
  }

  async function saveRecovery(e: SubmitEvent) {
    e.preventDefault();
    if (await onsetrecovery(question, answer)) {
      question = '';
      answer = '';
    }
  }
</script>

<Modal title="Settings" {onclose}>
  <section>
    <h3 class="text-sm font-semibold">Master password</h3>
    <p class="mt-1 text-sm text-zinc-500">Re-encrypts your vault with a new password.</p>
    <form onsubmit={changePassword} class="mt-4 flex gap-2">
      <Input type="password" placeholder="New master password" autocomplete="new-password" bind:value={newPassword} />
      <Button type="submit" class="shrink-0">Update</Button>
    </form>
    {#if passwordError}
      <p class="mt-2 text-sm text-rose-500">{passwordError}</p>
    {/if}
  </section>

  <section class="mt-8 border-t border-zinc-200 pt-6 dark:border-zinc-800">
    <h3 class="text-sm font-semibold">Recovery question</h3>
    <p class="mt-1 text-sm text-zinc-500">
      {#if recoveryQuestion}
        Current: <span class="font-medium text-zinc-700 dark:text-zinc-300">{recoveryQuestion}</span>. Saving again replaces it.
      {:else}
        Lets you reset the master password if you forget it. Keep the answer private.
      {/if}
    </p>
    <form onsubmit={saveRecovery} class="mt-4 space-y-2">
      <Input placeholder="Question, e.g. First pet's name?" autocomplete="off" required bind:value={question} />
      <div class="flex gap-2">
        <Input placeholder="Answer" autocomplete="off" required bind:value={answer} />
        <Button type="submit" class="shrink-0">Save</Button>
      </div>
    </form>
  </section>

  <section class="mt-8 border-t border-zinc-200 pt-6 dark:border-zinc-800">
    <h3 class="text-sm font-semibold text-rose-500">Delete vault</h3>
    <p class="mt-1 text-sm text-zinc-500">Permanently removes all saved passwords.</p>
    <div class="mt-4 flex gap-2">
      <Input placeholder="Type DELETE" autocomplete="off" bind:value={confirmText} />
      <Button variant="danger" class="shrink-0" disabled={!canDelete} onclick={() => ondelete()}>Delete</Button>
    </div>
  </section>
</Modal>
