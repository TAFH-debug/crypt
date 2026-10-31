<script lang="ts">
  import logo from '$lib/assets/logo.svg';
  import { AUTO_LOCK_MS, MIN_PASSWORD_LENGTH } from '$lib/password';
  import Button from './button.svelte';
  import Input from './input.svelte';
  import Stamp from './stamp.svelte';

  let {
    exists,
    question,
    error = '',
    notice = '',
    onunlock,
    onrecover
  }: {
    exists: boolean;
    question: string | null;
    error?: string;
    /** Why the vault is locked, e.g. after inactivity. */
    notice?: string;
    onunlock: (password: string) => Promise<void>;
    onrecover: (answer: string, password: string) => Promise<void>;
  } = $props();

  let mode = $state<'unlock' | 'recover'>('unlock');
  let password = $state('');
  let answer = $state('');
  let newPassword = $state('');
  let busy = $state(false);
  let shake = $state(false);
  let localError = $state('');

  const creating = $derived(!exists);
  const tab = $derived(mode === 'recover' ? 'Recovery' : creating ? 'New vault' : 'Locked');
  let shownError = $state('');
  const message = $derived(localError || shownError);

  function minLengthError(value: string): string {
    return value.length < MIN_PASSWORD_LENGTH ? `Use at least ${MIN_PASSWORD_LENGTH} characters.` : '';
  }

  async function submitUnlock(e: SubmitEvent) {
    e.preventDefault();
    localError = creating ? minLengthError(password) : '';
    if (localError) return;
    busy = true;
    await onunlock(password);
    busy = false;
  }

  async function submitRecover(e: SubmitEvent) {
    e.preventDefault();
    localError = minLengthError(newPassword);
    if (localError) return;
    busy = true;
    await onrecover(answer, newPassword);
    busy = false;
  }

  function switchMode(next: 'unlock' | 'recover') {
    localError = '';
    shownError = '';
    mode = next;
  }

  // Re-trigger the shake each time a new error arrives.
  $effect(() => {
    shownError = error;
    if (!error) return;
    shake = false;
    requestAnimationFrame(() => (shake = true));
  });
</script>

{#snippet label(text: string, forId: string)}
  <label for={forId} class="text-[11px] font-semibold tracking-[0.08em] text-muted uppercase">{text}</label>
{/snippet}

<main class="grid min-h-screen place-items-center bg-ground px-5 py-8">
  <div class={`relative w-full max-w-[360px] pt-6 ${shake ? 'animate-shake' : ''}`}>
    <!-- The card's guide tab names the state of the drawer. -->
    <span class="absolute top-0 left-6 grid h-6 place-items-center rounded-t-md bg-board px-3 text-xs font-extrabold tracking-[0.06em] text-board-ink uppercase">
      {tab}
    </span>

    <div class="rounded-b-lg border border-line border-t-2 border-t-board bg-card">
      <header class="px-7 pt-6 pb-5">
        <div class="flex items-center justify-between gap-3">
          <div class="flex items-center gap-3">
            <img src={logo} alt="" class="size-10" />
            <h1 class="text-[30px] leading-tight font-extrabold tracking-[-0.01em]">Crypt</h1>
          </div>
          {#if notice && mode === 'unlock'}
            <Stamp text={notice} />
          {/if}
        </div>
        <p class="mt-3 text-sm text-muted">
          {#if mode === 'recover'}
            Answer your recovery question to set a new master password.
          {:else if creating}
            Choose a master password. It unlocks this vault and never leaves this device.
          {:else}
            Enter your master password to open the vault.
          {/if}
        </p>
      </header>

      <div class="h-0.5 bg-rule" aria-hidden="true"></div>

      <div class="px-7 pt-5 pb-7">
        {#if mode === 'unlock'}
          <form onsubmit={submitUnlock} class="space-y-4">
            <div class="space-y-1.5">
              {@render label(creating ? 'New master password' : 'Master password', 'master')}
              <Input
                id="master"
                type="password"
                typed
                autocomplete={creating ? 'new-password' : 'current-password'}
                autofocus
                required
                aria-invalid={message ? 'true' : undefined}
                aria-describedby="lock-message"
                bind:value={password}
              />
            </div>
            <p id="lock-message" aria-live="polite" class="min-h-5 text-sm font-semibold text-weak">
              {#if message}{message}{:else}<span class="font-normal text-muted">{creating ? `At least ${MIN_PASSWORD_LENGTH} characters.` : `Locks again after ${AUTO_LOCK_MS / 60_000} minutes idle.`}</span>{/if}
            </p>
            <Button type="submit" variant="primary" class="w-full" disabled={busy}>
              {busy ? 'Opening…' : creating ? 'Create vault' : 'Unlock'}
            </Button>
          </form>

          {#if !creating && question}
            <button type="button" onclick={() => switchMode('recover')} class="mt-4 w-full text-center text-sm text-muted underline-offset-4 hover:text-ink hover:underline">
              Forgot your master password?
            </button>
          {/if}
        {:else}
          <form onsubmit={submitRecover} class="space-y-4">
            <p class="rounded-md bg-select px-3 py-2.5 text-sm font-semibold">{question}</p>
            <div class="space-y-1.5">
              {@render label('Your answer', 'recover-answer')}
              <Input id="recover-answer" typed autocomplete="off" spellcheck="false" autofocus required bind:value={answer} />
            </div>
            <div class="space-y-1.5">
              {@render label('New master password', 'recover-new')}
              <Input id="recover-new" type="password" typed autocomplete="new-password" required bind:value={newPassword} />
            </div>
            <p role="alert" class="min-h-5 text-sm font-semibold text-weak">{message}</p>
            <Button type="submit" variant="primary" class="w-full" disabled={busy}>{busy ? 'Resetting…' : 'Reset and unlock'}</Button>
          </form>

          <button type="button" onclick={() => switchMode('unlock')} class="mt-4 w-full text-center text-sm text-muted underline-offset-4 hover:text-ink hover:underline">
            Back to master password
          </button>
        {/if}
      </div>
    </div>
  </div>
</main>
