<script lang="ts">
  import type { HTMLInputAttributes } from 'svelte/elements';

  let {
    value = $bindable(''),
    class: className = '',
    ...rest
  }: Omit<HTMLInputAttributes, 'value'> & { value?: string; class?: string } = $props();
</script>

<input
  {...rest}
  bind:value
  class={`w-full rounded-xl bg-white px-3.5 py-2.5 text-sm text-zinc-900 ring-1 ring-zinc-200 transition outline-none placeholder:text-zinc-400 focus:ring-2 focus:ring-accent-1/60 dark:bg-zinc-900 dark:text-zinc-100 dark:ring-zinc-700 ${className}`}
/>
