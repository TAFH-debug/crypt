<script lang="ts">
  import { strength } from '$lib/password';

  let { password }: { password: string } = $props();

  const result = $derived(strength(password));
  const colors = ['bg-rose-500', 'bg-rose-500', 'bg-amber-400', 'bg-lime-500', 'bg-emerald-500'];
</script>

{#if result.score >= 0}
  <div class="flex items-center gap-2" aria-live="polite">
    <div class="flex flex-1 gap-1">
      {#each [0, 1, 2, 3] as i}
        <span class={`h-1 flex-1 rounded-full transition-colors duration-300 ${i < result.score ? colors[result.score] : 'bg-zinc-200 dark:bg-zinc-700'}`}></span>
      {/each}
    </div>
    <span class="w-12 text-right text-xs font-medium text-zinc-500">{result.label}</span>
  </div>
{/if}
