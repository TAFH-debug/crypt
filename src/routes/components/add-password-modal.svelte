<script lang="ts">
  import Modal from './modal.svelte';

  let { close, add } = $props();
  
  let newSite = $state('');
  let newUsername = $state('');
  let newPassword = $state('');
  let newNotes = $state('');
  let showPassword = $state(false);

  function generatePassword(length = 16) {
    const charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()_+";
    let password = "";
    for (let i = 0; i < length; i++) {
      const randomIndex = Math.floor(Math.random() * charset.length);
      password += charset[randomIndex];
    }
    return password;
  }


  function addPassword(e: any) {
    e.preventDefault();

    if (!newSite || !newUsername || !newPassword) return;
    
    const newEntry = {
      id: Date.now(),
      site: newSite,
      username: newUsername,
      password: newPassword,
      notes: newNotes,
      date: new Date().toISOString()
    };
    
    add(newEntry);
    resetForm();
  }

  function resetForm() {
    newSite = '';
    newUsername = '';
    newPassword = '';
    newNotes = '';
    showPassword = false;
  }

  function closeAction() {
    resetForm();
    close();
  }
</script>

<Modal title="Add New Password" close={closeAction}>
  <form onsubmit={addPassword}>
    <div class="space-y-4">
      <div>
        <label for="site" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Website or App</label>
        <input 
          type="text" 
          id="site"
          bind:value={newSite}
          required
          placeholder="example.com" 
          class="w-full px-3 py-2 rounded-lg border border-gray-300 dark:border-gray-600 focus:outline-none focus:ring-2 focus:ring-purple-500 bg-white dark:bg-gray-700 text-gray-800 dark:text-white"
        />
      </div>
      
      <div>
        <label for="username" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Username or Email</label>
        <input 
          type="text" 
          id="username"
          bind:value={newUsername}
          required
          placeholder="johndoe@example.com" 
          class="w-full px-3 py-2 rounded-lg border border-gray-300 dark:border-gray-600 focus:outline-none focus:ring-2 focus:ring-purple-500 bg-white dark:bg-gray-700 text-gray-800 dark:text-white"
        />
      </div>
      
      <div>
        <label for="password" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Password</label>
        <div class="relative">
          <input 
            type={showPassword ? "text" : "password"}
            id="password"
            bind:value={newPassword}
            required
            placeholder="Enter password" 
            class="w-full px-3 py-2 rounded-lg border border-gray-300 dark:border-gray-600 focus:outline-none focus:ring-2 focus:ring-purple-500 bg-white dark:bg-gray-700 text-gray-800 dark:text-white pr-24"
          />
          <div class="absolute right-2 top-2 flex">
            <button 
              type="button"
              onclick={() => showPassword = !showPassword}
              class="p-1 text-gray-400 hover:text-gray-600 dark:hover:text-gray-200"
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
              type="button"
              onclick={() => { newPassword = generatePassword(); }}
              class="ml-1 text-xs bg-gray-100 dark:bg-gray-600 text-gray-700 dark:text-gray-300 px-2 py-1 rounded hover:bg-gray-200 dark:hover:bg-gray-500 transition-colors"
            >
              Generate
            </button>
          </div>
        </div>
      </div>
      
      <div>
        <label for="notes" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Notes (Optional)</label>
        <textarea 
          id="notes"
          bind:value={newNotes}
          placeholder="Add any additional information" 
          rows="3"
          class="w-full px-3 py-2 rounded-lg border border-gray-300 dark:border-gray-600 focus:outline-none focus:ring-2 focus:ring-purple-500 bg-white dark:bg-gray-700 text-gray-800 dark:text-white resize-none"
        ></textarea>
      </div>
    </div>
    
    <div class="mt-6 flex justify-end space-x-3">
      <button 
        type="button"
        onclick={close}
        class="px-4 py-2 rounded-lg border border-gray-300 dark:border-gray-600 text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
      >
        Cancel
      </button>
      <button 
        type="submit"
        class="px-4 py-2 rounded-lg bg-gradient-to-r from-purple-500 to-indigo-600 text-white hover:opacity-90 transition-opacity"
      >
        Save Password
      </button>
    </div>
  </form>
</Modal>