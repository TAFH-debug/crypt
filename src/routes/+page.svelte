<script lang="ts">
  import PasswordList from './components/password-list.svelte';
  import AddPasswordModal from './components/add-password-modal.svelte';
  import ViewPasswordModal from './components/view-password-modal.svelte';
  import { invoke } from '@tauri-apps/api/core';
  import MasterPasswordModal from './components/master-password-modal.svelte';
  import SettingsButton from './components/settings-button.svelte';
  import SettingsModal from './components/settings-modal.svelte';
  import Input from './components/input.svelte';

  interface Password {
    id: number;
    site: string;
    username: string;
    password: string;
    notes: string;
  }

  let masterPassword = $state("");
  let filteredPasswords: Password[] = $state([]);
  let passwords: Password[] = $state([]);
  let showAddModal = $state(false);
  let showViewModal = $state(false);
  let showMasterPasswordModal = $state(true);
  let showSettingsModal = $state(false);
  let searchQuery = $state('');
  let currentPassword: Password | null = $state(null);
  let darkMode = $state(false);
  let error = $state("");

  let toast = $state({ show: false, message: '' });
  
  $effect(() => {
    filteredPasswords = passwords.filter(p => 
      p.site.toLowerCase().includes(searchQuery.toLowerCase()) || 
      p.username.toLowerCase().includes(searchQuery.toLowerCase()) ||
      p.notes.toLowerCase().includes(searchQuery.toLowerCase())
    );
  })

  function addPassword(password: Password) {
    console.log(passwords);
    passwords = [...passwords, password];
    showAddModal = false;
    saveStore();
  }

  function deletePassword(id: number) {
    passwords = passwords.filter(p => p.id !== id);
    if (showViewModal && currentPassword && currentPassword.id === id) {
      showViewModal = false;
      currentPassword = null;
    }
    saveStore();
  }

  function viewPassword(password: Password) {
    currentPassword = password;
    showViewModal = true;
  }

  function showToast(message: string, duration = 2000) {
    toast = { show: true, message };
    setTimeout(() => {
      toast = { show: false, message: '' };
    }, duration);
  }

  function copyToClipboard(text: string) {
    navigator.clipboard.writeText(text)
      .then(() => {
        showToast('Copied to clipboard!');
      })
      .catch(err => {
        showToast('Failed to copy!');
      });
  }

  async function saveStore() {
    await invoke('save_store', { store: passwords, password: masterPassword });
  }

  async function getStore() {
    const res: any = await invoke('get_store', { password: masterPassword });
    error = '';
    if (typeof res === 'string') {
      masterPassword = "";
      error = 'Invalid master password!';
      showMasterPasswordModal = true;
      return;
    }
    passwords = res;
  }

  async function deleteDatabase() {
    await invoke('delete_database');
    showMasterPasswordModal = true;
    masterPassword = "";
    passwords = [];
  }
</script>

<div class={darkMode ? 'dark' : ''}>
  <div class="min-h-screen bg-gray-50 dark:bg-gray-900 transition-colors duration-300">
    <section class="py-12 bg-gradient-to-b from-white to-gray-100 dark:from-gray-800 dark:to-gray-900">
      <div class="container mx-auto px-4 text-center">
        <h2 class="text-4xl font-bold mb-4 text-gray-800 dark:text-white">Crypt</h2>
        <div class="flex justify-center items-center space-x-4">
          <div class="relative w-full max-w-md">
            <Input 
              type="text" 
              bind:value={searchQuery}
              placeholder="Search your passwords..." 
              class="w-full px-4 py-3 rounded-lg border border-gray-300 dark:border-gray-600 focus:outline-none focus:ring-2 focus:ring-purple-500 bg-white dark:bg-gray-700 text-gray-800 dark:text-white"
            />
            <div class="absolute right-3 top-3 text-gray-400">
              <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
              </svg>
            </div>
          </div>
          <button 
            onclick={() => { showAddModal = true; }}
            class="bg-gradient-to-r from-purple-500 to-indigo-600 text-white px-4 py-2 rounded-lg hover:opacity-90 transition-opacity flex items-center"
          >
            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-1" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
            </svg>
            Add Password
          </button>
          <SettingsButton class="w-7 h-7" onclick={() => showSettingsModal = true}/>
        </div>
      </div>
    </section>

    <PasswordList 
      passwords={filteredPasswords} 
      searchQuery={searchQuery}
      view={viewPassword}
      delete={deletePassword}
      copy={copyToClipboard}
      add={() => { showAddModal = true; }}
    />

    {#if showAddModal}
      <AddPasswordModal 
        close={() => { showAddModal = false; }}
        add={addPassword}
      />
    {/if}

    {#if showViewModal && currentPassword}
      <ViewPasswordModal 
        password={currentPassword}
        close={() => { showViewModal = false; currentPassword = null; }}
        deleteA={deletePassword}
        copy={copyToClipboard}
      />
    {/if}

    {#if showMasterPasswordModal}
    <MasterPasswordModal 
      bind:masterPassword={masterPassword}
      bind:error={error}
      close={() => {
        showMasterPasswordModal = false;
        getStore();
      }}
    />
    {/if}

    {#if showSettingsModal}
    <SettingsModal close={() => {
      showSettingsModal = false;
    }}
    masterPassword={masterPassword}
    setMasterPassword={(val: string) => {
      masterPassword = val;
      saveStore();
    }}
    deleteDatabase={deleteDatabase}
    />
    {/if}
  </div>
</div>

<style>
  :global(body) {
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
  }
  
  :global(.dark) {
    color-scheme: dark;
  }
</style>