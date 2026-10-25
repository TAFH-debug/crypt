<script lang="ts">
  import type { Snippet } from 'svelte';
  import type { HTMLButtonAttributes } from 'svelte/elements';

  type Variant = 'primary' | 'secondary' | 'danger' | 'ghost';

  let {
    variant = 'secondary',
    class: className = '',
    children,
    ...rest
  }: HTMLButtonAttributes & { variant?: Variant; class?: string; children: Snippet } = $props();

  const variants: Record<Variant, string> = {
    primary:
      'bg-linear-to-r from-accent-1 to-accent-2 text-white shadow-md shadow-accent-1/25 hover:brightness-110 hover:shadow-lg hover:shadow-accent-2/30 active:scale-[0.98]',
    secondary:
      'bg-white text-zinc-700 shadow-sm ring-1 ring-zinc-200 hover:bg-zinc-50 hover:ring-zinc-300 dark:bg-zinc-800 dark:text-zinc-200 dark:ring-zinc-700 dark:hover:bg-zinc-700',
    danger: 'bg-rose-500 text-white shadow-md shadow-rose-500/25 hover:bg-rose-600 active:scale-[0.98]',
    ghost: 'text-zinc-500 hover:bg-zinc-200/60 hover:text-zinc-900 dark:text-zinc-400 dark:hover:bg-zinc-800 dark:hover:text-zinc-100'
  };
</script>

<button
  {...rest}
  class={`inline-flex items-center justify-center gap-2 rounded-xl px-4 py-2 text-sm font-semibold transition-all duration-150 disabled:cursor-not-allowed disabled:opacity-40 ${variants[variant]} ${className}`}
>
  {@render children()}
</button>
