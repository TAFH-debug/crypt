<script lang="ts">
  import Modal from './modal.svelte';
  let { close, error = $bindable(''), masterPassword = $bindable('') } = $props();

  function saveMasterPassword(e: any) {
    e.preventDefault();
    if (masterPassword.length < 8) {
      error = "Password must be at least 8 characters long";
      return;
    }
    close();
  }
</script>

<Modal title="Enter the master password" close={close}>
  <div>
    <label for="password" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Master password</label>
    <input 
      type="password" 
      id="password"
      bind:value={masterPassword}
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
</Modal>