<script lang="ts">
  import { MIN_PASSWORD_LENGTH } from '$lib/password';
  import Button from './button.svelte';
  import Input from './input.svelte';
  import Modal from './modal.svelte';

  let {
    onchangepassword,
    ondelete,
    onclose
  }: {
    onchangepassword: (password: string) => Promise<void>;
    ondelete: () => Promise<void>;
    onclose: () => void;
  } = $props();

  let newPassword = $state('');
  let passwordError = $state('');
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
</script>

<Modal title="Settings" {onclose}>
  <section>
    <h3 class="text-sm font-medium">Master password</h3>
    <p class="mt-1 text-sm text-zinc-500">Re-encrypts your vault with a new password.</p>
    <form onsubmit={changePassword} class="mt-4 flex gap-2">
      <Input type="password" placeholder="New master password" autocomplete="new-password" bind:value={newPassword} />
      <Button type="submit" class="shrink-0">Update</Button>
    </form>
    {#if passwordError}
      <p class="mt-2 text-sm text-red-600 dark:text-red-400">{passwordError}</p>
    {/if}
  </section>

  <section class="mt-8 border-t border-zinc-100 pt-6 dark:border-zinc-800">
    <h3 class="text-sm font-medium text-red-600 dark:text-red-400">Delete vault</h3>
    <p class="mt-1 text-sm text-zinc-500">Permanently removes all saved passwords.</p>
    <div class="mt-4 flex gap-2">
      <Input placeholder="Type DELETE" autocomplete="off" bind:value={confirmText} />
      <Button variant="danger" class="shrink-0" disabled={!canDelete} onclick={() => ondelete()}>Delete</Button>
    </div>
  </section>
</Modal>
