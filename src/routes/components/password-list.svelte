<script lang='ts'>
    import { fade, fly } from 'svelte/transition';
    import { flip } from 'svelte/animate';

    let props = $props();

    let passwords = $state(props.passwords);
    let searchQuery = $state(props.searchQuery);

    $effect(() => {
      passwords = props.passwords;
      searchQuery = props.searchQuery;
    });
    
    let showPassword: any = $state({});
  
    function togglePasswordVisibility(id: number) {
      showPassword[id] = !showPassword[id];
      showPassword = showPassword;
    }
  </script>
  
  <section class="py-8">
    <div class="container mx-auto px-4">
      {#if passwords.length === 0}
        <div class="text-center py-16" in:fade>
          <svg xmlns="http://www.w3.org/2000/svg" class="h-16 w-16 mx-auto text-gray-400 dark:text-gray-600 mb-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z" />
          </svg>
          {#if searchQuery}
            <h3 class="text-xl font-semibold text-gray-700 dark:text-gray-300">No passwords match your search</h3>
            <p class="text-gray-500 dark:text-gray-400 mt-2">Try a different search term or clear your search</p>
          {:else}
            <h3 class="text-xl font-semibold text-gray-700 dark:text-gray-300">No passwords saved yet</h3>
            <p class="text-gray-500 dark:text-gray-400 mt-2">Add your first password to get started</p>
            <button 
              onclick={() => props.add()}
              class="mt-4 bg-gradient-to-r from-purple-500 to-indigo-600 text-white px-6 py-2 rounded-lg hover:opacity-90 transition-opacity"
            >
              Add Password
            </button>
          {/if}
        </div>
      {:else}
        <div class="bg-white dark:bg-gray-800 rounded-xl shadow-lg overflow-hidden">
          <div class="overflow-x-auto">
            <table class="w-full">
              <thead>
                <tr class="bg-gray-50 dark:bg-gray-700 text-left">
                  <th class="px-6 py-3 text-gray-500 dark:text-gray-300 font-medium">Website</th>
                  <th class="px-6 py-3 text-gray-500 dark:text-gray-300 font-medium">Username</th>
                  <th class="px-6 py-3 text-gray-500 dark:text-gray-300 font-medium">Password</th>
                  <th class="px-6 py-3 text-gray-500 dark:text-gray-300 font-medium">Actions</th>
                </tr>
              </thead>
              <tbody>
                {#each passwords as password, i (password.id)}
                  <tr 
                    animate:flip={{ duration: 300 }}
                    in:fly={{ y: 20, duration: 300, delay: i * 50 }}
                    class="border-t border-gray-100 dark:border-gray-700 hover:bg-gray-400 dark:hover:bg-gray-750"
                  >
                    <td class="px-6 py-4">
                      <div class="flex items-center">
                        <div class="w-8 h-8 rounded bg-gray-200 dark:bg-gray-700 flex items-center justify-center text-gray-700 dark:text-gray-300 font-bold mr-3">
                          {password.site.charAt(0).toUpperCase()}
                        </div>
                        <span class="font-medium text-gray-800 dark:text-white">{password.site}</span>
                      </div>
                    </td>
                    <td class="px-6 py-4 text-gray-800 dark:text-gray-200">
                      <div class="flex items-center">
                        <span>{password.username}</span>
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
                    </td>
                    <td class="px-6 py-4 text-gray-800 dark:text-gray-200">
                      <div class="flex items-center">
                        <span>{showPassword[password.id] ? password.password : '••••••••••••'}</span>
                        <button 
                          onclick={() => togglePasswordVisibility(password.id)}
                          class="ml-2 text-gray-400 hover:text-gray-600 dark:hover:text-gray-200"
                          aria-label={showPassword[password.id] ? "Hide password" : "Show password"}
                        >
                          {#if showPassword[password.id]}
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
                    </td>
                    <td class="px-6 py-4">
                      <div class="flex space-x-2">
                        <button 
                          onclick={() => props.view(password)}
                          class="p-1 rounded-full hover:bg-gray-200 dark:hover:bg-gray-700 text-gray-500 dark:text-gray-400 hover:text-indigo-600 dark:hover:text-indigo-400 transition-colors"
                          aria-label="View details"
                        >
                          <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" />
                          </svg>
                        </button>
                        <button 
                          onclick={() => props.delete(password.id)}
                          class="p-1 rounded-full hover:bg-gray-200 dark:hover:bg-gray-700 text-gray-500 dark:text-gray-400 hover:text-red-600 dark:hover:text-red-400 transition-colors"
                          aria-label="Delete"
                        >
                          <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                          </svg>
                        </button>
                      </div>
                    </td>
                  </tr>
                {/each}
              </tbody>
            </table>
          </div>
        </div>
      {/if}
    </div>
  </section>