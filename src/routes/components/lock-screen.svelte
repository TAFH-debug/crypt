<script lang="ts">
  import { MIN_PASSWORD_LENGTH } from '$lib/password';
  import Button from './button.svelte';
  import Icon from './icon.svelte';
  import Input from './input.svelte';

  let { error = '', onunlock }: { error?: string; onunlock: (password: string) => Promise<void> } = $props();

  let password = $state('');
  let busy = $state(false);
  let shake = $state(false);
  let localError = $state('');

  async function submit(e: SubmitEvent) {
    e.preventDefault();
    if (password.length < MIN_PASSWORD_LENGTH) {
      localError = `Use at least ${MIN_PASSWORD_LENGTH} characters`;
      return;
    }
    localError = '';
    busy = true;
    await onunlock(password);
    busy = false;
  }

  // Re-trigger the shake each time a new error arrives.
  $effect(() => {
    if (!error) return;
    shake = false;
    requestAnimationFrame(() => (shake = true));
  });
</script>

<main class="relative flex min-h-screen items-center justify-center overflow-hidden px-5">
  <div class="pointer-events-none absolute inset-0 -z-10">
    <div class="animate-drift absolute top-1/4 left-1/4 h-72 w-72 rounded-full bg-accent-1/40 blur-3xl"></div>
    <div class="animate-drift absolute right-1/4 bottom-1/4 h-64 w-64 rounded-full bg-accent-2/40 blur-3xl [animation-delay:-5s]"></div>
    <div class="absolute top-1/2 left-1/2 h-56 w-56 -translate-x-1/2 -translate-y-1/2 rounded-full bg-accent-3/30 blur-3xl"></div>
  </div>

  <form
    onsubmit={submit}
    class={`w-full max-w-xs space-y-5 rounded-3xl bg-white/70 p-7 shadow-2xl ring-1 ring-black/5 backdrop-blur-xl dark:bg-zinc-900/70 dark:ring-white/10 ${shake ? 'animate-[shake_0.4s]' : ''}`}
  >
    <div class="text-center">
      <div class="mx-auto mb-4 flex h-14 w-14 items-center justify-center rounded-2xl bg-linear-to-br from-accent-1 to-accent-2 text-white shadow-lg shadow-accent-1/30">
        <Icon name="lock" class="h-6 w-6" />
      </div>
      <h1 class="text-2xl font-extrabold tracking-tight">Crypt</h1>
      <p class="mt-1 text-sm text-zinc-500">Enter your master password</p>
    </div>

    <Input
      type="password"
      aria-label="Master password"
      placeholder="Master password"
      autocomplete="off"
      autofocus
      required
      bind:value={password}
    />

    <p class="min-h-5 text-center text-sm text-rose-500">{localError || error}</p>

    <Button type="submit" variant="primary" class="w-full" disabled={busy}>Unlock</Button>
  </form>
</main>
