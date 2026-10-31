<script lang="ts" module>
  const icons = {
    search: ['M21 21l-5.2-5.2M17 10.5a6.5 6.5 0 11-13 0 6.5 6.5 0 0113 0z'],
    plus: ['M12 5v14M5 12h14'],
    copy: ['M9 9h10v10H9z', 'M5 15V5h10'],
    check: ['M5 13l4 4L19 7'],
    eye: ['M2 12s3.6-7 10-7 10 7 10 7-3.6 7-10 7S2 12 2 12z', 'M12 15a3 3 0 100-6 3 3 0 000 6z'],
    'eye-off': [
      'M10.6 5.1A10.6 10.6 0 0112 5c6.4 0 10 7 10 7a17.6 17.6 0 01-2.9 3.9M6.6 6.6C3.6 8.5 2 12 2 12s3.6 7 10 7a10 10 0 005.4-1.6',
      'M9.9 9.9a3 3 0 004.2 4.2',
      'M3 3l18 18'
    ],
    lock: ['M6 11h12v10H6z', 'M8.5 11V7.5a3.5 3.5 0 017 0V11'],
    refresh: ['M20 11a8 8 0 00-14.6-4.5L4 8', 'M4 4v4h4', 'M4 13a8 8 0 0014.6 4.5L20 16', 'M20 20v-4h-4'],
    settings: [
      'M12 15a3 3 0 100-6 3 3 0 000 6z',
      'M19.4 15a1.7 1.7 0 00.3 1.8l.1.1a2 2 0 11-2.8 2.8l-.1-.1a1.7 1.7 0 00-1.8-.3 1.7 1.7 0 00-1 1.5V21a2 2 0 11-4 0v-.1a1.7 1.7 0 00-1.1-1.5 1.7 1.7 0 00-1.8.3l-.1.1a2 2 0 11-2.8-2.8l.1-.1a1.7 1.7 0 00.3-1.8 1.7 1.7 0 00-1.5-1H3a2 2 0 110-4h.1a1.7 1.7 0 001.5-1.1 1.7 1.7 0 00-.3-1.8l-.1-.1a2 2 0 112.8-2.8l.1.1a1.7 1.7 0 001.8.3H9a1.7 1.7 0 001-1.5V3a2 2 0 114 0v.1a1.7 1.7 0 001 1.5 1.7 1.7 0 001.8-.3l.1-.1a2 2 0 112.8 2.8l-.1.1a1.7 1.7 0 00-.3 1.8V9a1.7 1.7 0 001.5 1H21a2 2 0 110 4h-.1a1.7 1.7 0 00-1.5 1z'
    ],
    'arrow-left': ['M19 12H5M11 18l-6-6 6-6'],
    'arrow-up': ['M12 19V5M6 11l6-6 6 6'],
    'arrow-down': ['M12 5v14M6 13l6 6 6-6'],
    file: ['M14 3H7a2 2 0 00-2 2v14a2 2 0 002 2h10a2 2 0 002-2V8z', 'M14 3v5h5'],
    paperclip: ['M21.4 11.1l-9.2 9.1a6 6 0 01-8.5-8.5l9.2-9.1a4 4 0 015.7 5.6l-9.2 9.2a2 2 0 01-2.8-2.8l8.5-8.5'],
    download: ['M12 4v11', 'M7 10l5 5 5-5', 'M5 20h14']
  };

  export type IconName = keyof typeof icons;
</script>

<script lang="ts">
  let { name, class: className = 'size-4' }: { name: IconName; class?: string } = $props();
</script>

<svg
  xmlns="http://www.w3.org/2000/svg"
  class={className}
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="1.75"
  stroke-linecap="round"
  stroke-linejoin="round"
  aria-hidden="true"
>
  {#each icons[name] as d}
    <path {d} />
  {/each}
</svg>
