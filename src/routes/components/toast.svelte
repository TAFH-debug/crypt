<script lang="ts">
  import { fade } from 'svelte/transition';

  let { message }: { message: string } = $props();
</script>

<div
  role="status"
  transition:fade={{ duration: 150 }}
  class="pointer-events-none fixed bottom-6 left-1/2 z-[60] -translate-x-1/2 rounded-full bg-zinc-900 px-4 py-2 text-sm text-white shadow-lg dark:bg-zinc-100 dark:text-zinc-900"
>
  {message}
</div>
