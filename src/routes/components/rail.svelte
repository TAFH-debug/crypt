<script lang="ts">
  import logo from '$lib/assets/logo-light.svg';
  import Icon, { type IconName } from './icon.svelte';

  let {
    pane,
    onadd,
    onfiles,
    onsettings,
    onlock
  }: {
    pane: 'entry' | 'new' | 'files' | 'settings';
    onadd: () => void;
    onfiles: () => void;
    onsettings: () => void;
    onlock: () => void;
  } = $props();

  const mod = typeof navigator !== 'undefined' && /Mac/.test(navigator.platform) ? '⌘' : 'Ctrl+';
</script>

{#snippet railButton(icon: IconName, label: string, onclick: () => void, current: boolean)}
  <button
    type="button"
    title={label}
    aria-label={label}
    aria-current={current ? 'page' : undefined}
    {onclick}
    class={`grid size-10 place-items-center rounded-lg transition-colors duration-150 focus-visible:outline-board-ink ${current ? 'bg-board-ink/15 text-board-ink' : 'text-board-muted hover:bg-board-ink/10 hover:text-board-ink'}`}
  >
    <Icon name={icon} class="size-5" />
  </button>
{/snippet}

<nav aria-label="Vault" class="flex h-full flex-col items-center gap-2 bg-board py-4">
  <img src={logo} alt="Crypt" class="mb-4 size-9" />

  <button
    type="button"
    title={`New login (${mod}N)`}
    aria-label="New login"
    aria-current={pane === 'new' ? 'page' : undefined}
    onclick={onadd}
    class="grid size-10 place-items-center rounded-lg bg-board-ink text-board transition-transform duration-150 hover:scale-105 focus-visible:outline-board-ink active:scale-95"
  >
    <Icon name="plus" class="size-5" />
  </button>

  {@render railButton('file', 'Files', onfiles, pane === 'files')}

  <div class="flex-1"></div>

  {@render railButton('settings', 'Vault settings', onsettings, pane === 'settings')}
  {@render railButton('lock', 'Lock vault', onlock, false)}
</nav>
