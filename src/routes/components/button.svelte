<script lang="ts">
  import type { Snippet } from 'svelte';
  import type { HTMLButtonAttributes } from 'svelte/elements';

  type Variant = 'primary' | 'secondary' | 'danger' | 'ghost';

  let {
    variant = 'secondary',
    type = 'button',
    class: className = '',
    children,
    ...rest
  }: HTMLButtonAttributes & { variant?: Variant; class?: string; children: Snippet } = $props();

  const variants: Record<Variant, string> = {
    primary: 'bg-board text-board-ink hover:bg-board-hover active:translate-y-px',
    secondary: 'border border-line bg-card text-ink hover:border-active/50 hover:bg-select/60 active:translate-y-px',
    danger: 'bg-ink text-card hover:bg-ink/85 active:translate-y-px',
    ghost: 'text-muted hover:bg-select hover:text-ink'
  };
</script>

<button
  {...rest}
  {type}
  class={`inline-flex h-10 shrink-0 items-center justify-center gap-2 rounded-lg px-4 text-sm font-semibold transition-colors duration-150 disabled:pointer-events-none disabled:opacity-45 ${variants[variant]} ${className}`}
>
  {@render children()}
</button>
