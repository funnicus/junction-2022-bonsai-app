<script lang="ts">
	import type { Data } from "../dataSchema";
  export let node: Data

  const x2 = Math.sin(node?.angle ?? 0) * (node?.length ?? 0)
  const y2 = Math.cos(node?.angle ?? 0) * (node?.length ?? 0)
</script>

<line x1="0" x2="0" y1="0" y2="{node.length}" stroke="red" stroke-width="2"  transform="rotate({-(node?.angle ?? 0) / Math.PI * 180})"/>
<g transform="translate({x2} {y2})">
  {#each node.children as child}
    <svelte:self node={child} />
  {/each}
</g>

