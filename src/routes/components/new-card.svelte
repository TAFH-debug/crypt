<script lang="ts">
  import { generatePassword, type Entry } from '$lib/password';
  import Button from './button.svelte';
  import CardShell from './card-shell.svelte';
  import Icon from './icon.svelte';
  import Input from './input.svelte';
  import StrengthMeter from './strength-meter.svelte';

  let {
    error,
    onsave,
    oncancel
  }: {
    error: string;
    onsave: (entry: Entry) => Promise<void>;
    oncancel: () => void;
  } = $props();

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

{#snippet label(text: string, forId: string)}
  <label for={forId} class="text-[11px] font-semibold tracking-[0.08em] text-muted uppercase">{text}</label>
{/snippet}

<CardShell title="New login" {error} onback={oncancel}>
  {#snippet sub()}Encrypted and saved only on this device.{/snippet}

  <form onsubmit={submit} class="mt-4 space-y-5">
    <div class="space-y-1.5">
      {@render label('Website or app', 'new-site')}
      <Input id="new-site" placeholder="github.com" autocomplete="off" autofocus required bind:value={site} />
    </div>

    <div class="space-y-1.5">
      {@render label('Username or email', 'new-user')}
      <Input id="new-user" typed placeholder="you@example.com" autocomplete="off" spellcheck="false" required bind:value={username} />
    </div>

    <div class="space-y-1.5">
      {@render label('Password', 'new-pass')}
      <div class="flex gap-2">
        <div class="relative flex-1">
          <Input
            id="new-pass"
            typed
            type={showPassword ? 'text' : 'password'}
            autocomplete="off"
            spellcheck="false"
            required
            bind:value={password}
            class="pr-11"
          />
          <button
            type="button"
            aria-label={showPassword ? 'Hide password' : 'Show password'}
            aria-pressed={showPassword}
            onclick={() => (showPassword = !showPassword)}
            class="absolute inset-y-0 right-1 my-auto grid size-8 place-items-center rounded-md text-muted hover:bg-select hover:text-active"
          >
            <Icon name={showPassword ? 'eye-off' : 'eye'} />
          </button>
        </div>
        <Button onclick={generate} title="Generate a 20-character password">
          <Icon name="refresh" />
          Generate
        </Button>
      </div>
      <div class="pt-1"><StrengthMeter {password} /></div>
    </div>

    <div class="space-y-1.5">
      {@render label('Notes', 'new-notes')}
      <textarea
        id="new-notes"
        rows="3"
        autocomplete="off"
        placeholder="Optional. Recovery hints, security questions, 2FA notes."
        bind:value={notes}
        class="block w-full resize-none rounded-lg border border-line bg-card px-3 py-2.5 text-sm text-ink transition-colors duration-150 outline-none placeholder:text-muted hover:border-active/50 focus:border-active focus:ring-3 focus:ring-active/20"
      ></textarea>
    </div>

    <div class="flex justify-end gap-2 pt-1">
      <Button variant="ghost" onclick={oncancel}>Cancel</Button>
      <Button type="submit" variant="primary" disabled={!ready || saving}>{saving ? 'Saving…' : 'Save login'}</Button>
    </div>
  </form>
</CardShell>
