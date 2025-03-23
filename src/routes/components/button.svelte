<script>
    let props = $props();
</script>

<button 
onclick={props.onclick}
disabled={props.disabled}
class={`px-4 py-2 rounded-lg border border-gray-300 dark:border-gray-600 text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors ${props.class}`}
>
{@render props.children?.()}
</button>