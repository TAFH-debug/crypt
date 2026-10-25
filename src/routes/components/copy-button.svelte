<script lang="ts">
  import Button from './button.svelte';
  import Icon from './icon.svelte';

  let { text, label, oncopy }: { text: string; label: string; oncopy: (text: string) => Promise<boolean> } = $props();

  let copied = $state(false);
  let timer: ReturnType<typeof setTimeout> | undefined;

  async function copy() {
    if (!(await oncopy(text))) return;
    copied = true;
    clearTimeout(timer);
    timer = setTimeout(() => (copied = false), 1500);
  }
</script>

<Button variant="ghost" class="p-2" title={label} aria-label={label} onclick={copy}>
  <span class={copied ? 'text-emerald-500' : ''}>
    <Icon name={copied ? 'check' : 'copy'} />
  </span>
</Button>
