<script>
  import { fade, scale } from "svelte/transition";
  const { title, children, close, ...props } = $props();
</script>

<div 
  class={`fixed inset-0 bg-black/75 flex items-center justify-center z-50 p-4 ${props.class}`}
  transition:fade={{ duration: 200 }}
>
  <div 
    class="bg-white dark:bg-gray-800 rounded-xl opacity-100 shadow-xl w-full max-w-md overflow-hidden"
    transition:scale={{ start: 0.95, duration: 200 }}
  >
    <div class="p-6">
      <div class="flex justify-between items-center mb-4">
        <h3 class="text-xl font-bold text-gray-800 dark:text-white">{title}</h3>
        <button 
          onclick={close}
          class="text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200"
          aria-label="Close"
        >
          <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>

      {@render children?.()}
    </div>
  </div>
</div>