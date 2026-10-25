<script lang="ts">
  import { generatePassword, type Entry } from '$lib/password';
  import Button from './button.svelte';
  import Icon from './icon.svelte';
  import Input from './input.svelte';
  import Modal from './modal.svelte';
  import StrengthMeter from './strength-meter.svelte';

  let { onsave, onclose }: { onsave: (entry: Entry) => Promise<void>; onclose: () => void } = $props();

  let site = $state('');
  let username = $state('');
  let password = $state('');
  let notes = $state('');
  let showPassword = $state(false);
  let saving = $state(false);

  const ready = $derived(site.trim() !== '' && username.trim() !== '' && password !== '');

  function generate() {
    password = generatePassword();
    showPassword = true; // show the result so it can be checked
  }

  async function submit(e: SubmitEvent) {
    e.preventDefault();
    if (!ready) return;

    saving = true;
    await onsave({
      id: Date.now(),
      site: site.trim(),
      username: username.trim(),
      password,
      notes: notes.trim(),
      date: new Date().toISOString()
    });
    saving = false;
  }
</script>

<Modal title="New password" {onclose}>
  <form onsubmit={submit} class="space-y-4">
    <label class="block space-y-1.5">
      <span class="text-xs font-semibold tracking-wide text-zinc-500 uppercase">Website or app</span>
      <Input type="text" placeholder="github.com" autocomplete="off" autofocus required bind:value={site} />
    </label>

    <label class="block space-y-1.5">
      <span class="text-xs font-semibold tracking-wide text-zinc-500 uppercase">Username or email</span>
      <Input type="text" placeholder="you@example.com" autocomplete="off" required bind:value={username} />
    </label>

    <div class="space-y-1.5">
      <span class="text-xs font-semibold tracking-wide text-zinc-500 uppercase">Password</span>
      <div class="relative">
        <Input
          type={showPassword ? 'text' : 'password'}
          autocomplete="off"
          required
          bind:value={password}
          class="pr-36 font-mono"
        />
        <div class="absolute inset-y-0 right-1.5 flex items-center gap-1">
          <Button variant="ghost" class="p-1.5" aria-label={showPassword ? 'Hide password' : 'Show password'} onclick={() => (showPassword = !showPassword)}>
            <Icon name={showPassword ? 'eye-off' : 'eye'} />
          </Button>
          <Button variant="secondary" class="gap-1 px-2.5 py-1.5 text-xs text-accent-1" onclick={generate}>
            <Icon name="sparkles" class="h-3.5 w-3.5" />
            Generate
          </Button>
        </div>
      </div>
      <StrengthMeter {password} />
    </div>

    <label class="block space-y-1.5">
      <span class="text-xs font-semibold tracking-wide text-zinc-500 uppercase">Notes</span>
      <textarea
        rows="3"
        autocomplete="off"
        placeholder="Optional"
        bind:value={notes}
        class="w-full resize-none rounded-xl bg-white px-3.5 py-2.5 text-sm ring-1 ring-zinc-200 transition outline-none placeholder:text-zinc-400 focus:ring-2 focus:ring-accent-1/60 dark:bg-zinc-900 dark:ring-zinc-700"
      ></textarea>
    </label>

    <div class="flex justify-end gap-2 pt-2">
      <Button variant="ghost" onclick={onclose}>Cancel</Button>
      <Button type="submit" variant="primary" disabled={!ready || saving}>Save</Button>
    </div>
  </form>
</Modal>
