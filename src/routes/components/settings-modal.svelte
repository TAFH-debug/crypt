<script>
  import Button from "./button.svelte";
  import Input from "./input.svelte";
  import Modal from "./modal.svelte";
  let { close, setMasterPassword, masterPassword, deleteDatabase } = $props();

  let newMasterPassword = $state(masterPassword);
  let error = $state("");
  let deleteDisabled = $state(true);

  function setPassword() {
    if (newMasterPassword.length < 8) {
        error = "Password must be at least 8 characters long";
        return;
    }
    setMasterPassword(newMasterPassword);
    close();
  }

  let confirmValue = $state("");
  $effect(() => {
    deleteDisabled = confirmValue !== "I confirm";
  });
</script>

<Modal title="Settings" close={close} class="text-gray-800 dark:text-white">
    <section class="mx-2 my-4">
        <div>
            <h2 class="font-medium my-2 w-full">Master Password</h2>
            <div class="h-[1px] bg-gray-400 flex-1 my-2"></div>
        </div>
        <Input class="mt-3 mb-2" bind:value={newMasterPassword} />
        <div class="text-red-500 mb-2">
            {error}
        </div>
        <Button onclick={setPassword} class="bg-red-500 hover:bg-red-400 hover:dark:bg-red-400">
            Change master password
        </Button>
    </section>

    <section class="mx-2 my-4">
        <div>
            <h2 class="text-gray-800 dark:text-white font-medium my-2 w-full">Password database</h2>
            <div class="h-[1px] bg-gray-400 flex-1 my-2"></div>
        </div>
        <label for="confirm" class="my-3">Enter "I confirm" in the following input:</label>
        <Input id="confirm" class="my-3" placeholder="I confirm" bind:value={confirmValue} />
        <Button onclick={deleteDatabase} class={deleteDisabled ? "bg-red-400 hover:bg-red-400 hover:dark:bg-red-400" : "bg-red-500 hover:bg-red-400 hover:dark:bg-red-400"} disabled={deleteDisabled}>
            Delete database
        </Button>
    </section>
</Modal>