<script lang="ts">
  import type { HTMLInputAttributes } from 'svelte/elements';

  let {
    value = $bindable(''),
    ref = $bindable(),
    typed = false,
    class: className = '',
    ...rest
  }: Omit<HTMLInputAttributes, 'value'> & {
    value?: string;
    ref?: HTMLInputElement;
    /** Credential data is typed onto the card in the typewriter face. */
    typed?: boolean;
    class?: string;
  } = $props();
</script>

<input
  {...rest}
  bind:this={ref}
  bind:value
  class={`h-10 w-full rounded-lg border border-line bg-card px-3 text-ink transition-colors duration-150 outline-none placeholder:text-muted hover:border-active/50 focus:border-active focus:ring-3 focus:ring-active/20 ${typed ? 'font-type text-[15px]' : 'text-sm'} ${className}`}
/>
