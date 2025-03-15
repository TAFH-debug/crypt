<script>
    let { password, service } = $props();
</script>
<div class='flex text-white w-full rounded-full hover:bg-gray-200 transition-all duration-300 ease-in-out p-2 m-4 items-center justify-between'>
    <div class="m-2">
        Password from {service}
    </div>
    <button onclick={() => alert(password)} class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded-full ml-4">
        Show
    </button>
</div>
