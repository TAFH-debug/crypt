<script lang="ts" module>
  export type Tone = 'ink' | 'weak' | 'strong';
  export type StampState = { id: number; text: string; tone: Tone };
</script>

<script lang="ts">
  let { text, tone = 'ink', small = false }: { text: string; tone?: Tone; small?: boolean } = $props();

  const tones: Record<Tone, string> = {
    ink: 'border-active text-active',
    weak: 'border-weak text-weak',
    strong: 'border-strong text-strong'
  };
</script>

<span
  class={`inline-block shrink-0 -rotate-3 ${small ? '' : 'animate-stamp'} rounded-[3px] border-[1.5px] font-type leading-none font-bold tracking-[0.12em] whitespace-nowrap uppercase ${small ? 'px-1 py-0.5 text-[11px]' : 'px-2 py-1.5 text-[13px]'} ${tones[tone]}`}
>
  {text}
</span>
