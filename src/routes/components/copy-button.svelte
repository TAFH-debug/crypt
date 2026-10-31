<script lang="ts">
  import Icon from './icon.svelte';

  let {
    label,
    oncopy,
    class: className = ''
  }: { label: string; oncopy: () => Promise<boolean>; class?: string } = $props();

  let copied = $state(false);
  let timer: ReturnType<typeof setTimeout> | undefined;

  async function copy() {
    if (!(await oncopy())) return;
    copied = true;
    clearTimeout(timer);
    timer = setTimeout(() => (copied = false), 1500);
  }
</script>

<button
  type="button"
  title={label}
  aria-label={label}
  onclick={copy}
  class={`grid size-9 shrink-0 place-items-center rounded-lg transition-colors duration-150 hover:bg-select hover:text-active ${copied ? 'text-strong' : 'text-muted'} ${className}`}
>
  <Icon name={copied ? 'check' : 'copy'} class="size-[18px]" />
</button>
