<script lang="ts">
  import { strength } from '$lib/password';

  let { password }: { password: string } = $props();

  const result = $derived(strength(password));
  const fill = $derived(result.score < 2 ? 'bg-weak' : result.score === 2 ? 'bg-fair' : 'bg-strong');
  const text = $derived(result.score < 2 ? 'text-weak' : result.score === 2 ? 'text-fair' : 'text-strong');
</script>

{#if result.score >= 0}
  <div class="flex items-center gap-3" aria-live="polite">
    <div class="flex w-32 gap-1" aria-hidden="true">
      {#each [0, 1, 2, 3] as i}
        <span class={`h-1 flex-1 rounded-full transition-colors duration-300 ${i < Math.max(result.score, 1) ? fill : 'bg-hair'}`}></span>
      {/each}
    </div>
    <span class={`text-xs font-semibold ${text}`}>{result.label}</span>
  </div>
{/if}
