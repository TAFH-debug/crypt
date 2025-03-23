<script>
    let { value = $bindable(), ...props } = $props();
</script>

<input 
{...props}
bind:value={value}
class={`w-full px-3 py-2 rounded-lg border border-gray-300 dark:border-gray-600 focus:outline-none focus:ring-2 focus:ring-purple-500 bg-white dark:bg-gray-700 text-gray-800 dark:text-white ${props.class}`}
/>