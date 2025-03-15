<script>
    import { fade, scale } from 'svelte/transition';
    let props = $props();
    let password = props.password;

    let showPassword = $state(false);
    
    function togglePasswordVisibility() {
      showPassword = !showPassword;
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
          <h3 class="text-xl font-bold text-gray-800 dark:text-white">Password Details</h3>
          <button 
            onclick={() => props.close()}
            class="text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200"
            aria-label="Close"
          >
            <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </div>
        
        <div class="space-y-4">
          <div>
            <h4 class="text-sm font-medium text-gray-500 dark:text-gray-400">Website or App</h4>
            <p class="text-gray-800 dark:text-white text-lg">{password.site}</p>
          </div>
          
          <div>
            <h4 class="text-sm font-medium text-gray-500 dark:text-gray-400">Username or Email</h4>
            <div class="flex items-center">
              <p class="text-gray-800 dark:text-white text-lg">{password.username}</p>
              <button 
                onclick={() => props.copy(password.username)}
                class="ml-2 text-gray-400 hover:text-gray-600 dark:hover:text-gray-200"
                aria-label="Copy username"
              >
                <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 5H6a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2v-1M8 5a2 2 0 002 2h2a2 2 0 002-2M8 5a2 2 0 012-2h2a2 2 0 012 2m0 0h2a2 2 0 012 2v3m2 4H10m0 0l3-3m-3 3l3 3" />
                </svg>
              </button>
            </div>
          </div>
          
          <div>
            <h4 class="text-sm font-medium text-gray-500 dark:text-gray-400">Password</h4>
            <div class="flex items-center">
              <p class="text-gray-800 dark:text-white text-lg">{showPassword ? password.password : '••••••••••••'}</p>
              <button 
                onclick={togglePasswordVisibility}
                class="ml-2 text-gray-400 hover:text-gray-600 dark:hover:text-gray-200"
                aria-label={showPassword ? "Hide password" : "Show password"}
              >
                {#if showPassword}
                  <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13.875 18.825A10.05 10.05 0 0112 19c-4.478 0-8.268-2.943-9.543-7a9.97 9.97 0 011.563-3.029m5.858.908a3 3 0 114.243 4.243M9.878 9.878l4.242 4.242M9.88 9.88l-3.29-3.29m7.532 7.532l3.29 3.29M3 3l3.59 3.59m0 0A9.953 9.953 0 0112 5c4.478 0 8.268 2.943 9.543 7a10.025 10.025 0 01-4.132 5.411m0 0L21 21" />
                  </svg>
                {:else}
                  <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" />
                  </svg>
                {/if}
              </button>
              <button 
                onclick={() => props.copy(password.password)}
                class="ml-2 text-gray-400 hover:text-gray-600 dark:hover:text-gray-200"
                aria-label="Copy password"
              >
                <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 5H6a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2v-1M8 5a2 2 0 002 2h2a2 2 0 002-2M8 5a2 2 0 012-2h2a2 2 0 012 2m0 0h2a2 2 0 012 2v3m2 4H10m0 0l3-3m-3 3l3 3" />
                </svg>
              </button>
            </div>
          </div>
          
          {#if password.notes}
            <div>
              <h4 class="text-sm font-medium text-gray-500 dark:text-gray-400">Notes</h4>
              <p class="text-gray-800 dark:text-white">{password.notes}</p>
            </div>
          {/if}
          
          <div>
            <h4 class="text-sm font-medium text-gray-500 dark:text-gray-400">Date Added</h4>
            <p class="text-gray-800 dark:text-white">
              {new Date(password.date).toLocaleDateString()}
            </p>
          </div>
        </div>
        
        <div class="mt-6 flex justify-end space-x-3">
          <button 
            onclick={() => props.close()}
            class="px-4 py-2 rounded-lg border border-gray-300 dark:border-gray-600 text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
          >
            Close
          </button>
          <button 
            onclick={() => props.delete(password.id)}
            class="px-4 py-2 rounded-lg bg-red-500 text-white hover:bg-red-600 transition-colors flex items-center"
          >
            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 mr-1" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
            </svg>
            Delete
          </button>
        </div>
      </div>
    </div>
  </div>