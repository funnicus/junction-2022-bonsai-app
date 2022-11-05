<script lang="ts">
	import type { TreeStore } from "$lib/stores/tree";
	import { generateLeafPath } from "$lib/utils/path";
  import random from 'seedrandom';

  import { getContext } from 'svelte'

  const treeStore = getContext<TreeStore>('tree')

  export let depth: number
  export let width: number;

  const rnd = random(depth.toString()).double();

  const scale = 1 + ((1/depth) * 4)
</script>

{#if $treeStore.showLeaves}
<g style="z-index: 30">
  <polygon x="0" y="0" fill="#6C9D2D" points="{generateLeafPath(scale, depth.toString())}" />
  <polygon transform="translate(5, 5) scale(0.5) rotate({rnd*360})" fill="#74a339" points="{generateLeafPath(scale, (depth + 1).toString())}" />
  <polygon transform="translate(-5, -5) scale(0.75) rotate({-rnd*180})" fill="#6C9D2D" points="{generateLeafPath(scale, (depth + 1).toString())}" />
  <polygon transform="translate(-15, 5) scale(0.5) rotate({-rnd*180})" fill="#679133" points="{generateLeafPath(scale, (depth + 1).toString())}" />
</g>
{/if}

