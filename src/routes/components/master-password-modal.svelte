<script lang="ts">
    import { fade, scale } from 'svelte/transition';
    let props = $props();
    let master_password = $state("");
    let error = $state("");

    function saveMasterPassword(e: any) {
      e.preventDefault();
      if (master_password.length < 8) {
        error = "Password must be at least 8 characters long";
        return;
      }
      props.set(master_password);
    }
</script>
  
  <div 
    class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4"
    transition:fade={{ duration: 200 }}
  >
    <div 
      class="bg-white dark:bg-gray-800 rounded-xl shadow-xl w-full max-w-md overflow-hidden"
      transition:scale={{ start: 0.95, duration: 200 }}
    >
      <div class="p-6">
        <div class="flex justify-between items-center mb-4">
          <h3 class="text-xl font-bold text-gray-800 dark:text-white">Enter the master password: </h3>
        </div>

        <div>
          <label for="password" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Master password</label>
          <input 
            type="password" 
            id="password"
            bind:value={master_password}
            required
            placeholder="********" 
            class="w-full px-3 py-2 rounded-lg border border-gray-300 dark:border-gray-600 focus:outline-none focus:ring-2 focus:ring-purple-500 bg-white dark:bg-gray-700 text-gray-800 dark:text-white"
          />
        </div>
        <div class="text-red-500 py-2">
          {error}
        </div>
        
        <div class="mt-6 flex justify-end space-x-3">
          <button 
            onclick={saveMasterPassword}
            class="px-4 py-2 rounded-lg bg-gradient-to-r from-purple-500 to-indigo-600 text-white hover:opacity-90 transition-opacity"
          >
            Save Password
          </button>
        </div>
      </div>
    </div>
  </div>