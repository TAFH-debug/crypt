<script lang="ts">
  import Password from "./components/password.svelte";

  let name = $state("");
</script>

<main class="min-h-screen bg-gray-700">
  <h1 class="text-center w-full text-5xl text-white py-20">
    Password Manager
  </h1>
  <div class="w-4/5 flex items-center flex-col m-auto">
    <Password service={"Discord"} password={"123456789"}/>
  </div>
</main>

<style>

</style>
