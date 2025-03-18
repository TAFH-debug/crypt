<script>
    import { fly } from 'svelte/transition';
    
    let { message, children } = $props();
    
  </script>
  
  <div 
    class="fixed bottom-4 right-4 bg-gray-800 text-white px-4 py-2 rounded-lg shadow-lg z-50"
    transition:fly={{ y: 20, duration: 300 }}
  >
    {message}
    {@render children?.()}
  </div>