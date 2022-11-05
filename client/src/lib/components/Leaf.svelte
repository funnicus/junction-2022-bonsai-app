<script lang="ts">
	import Leaf1 from "$lib/icons/Leaf1.svelte";
	import Leaf2 from "$lib/icons/Leaf2.svelte";
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
  <g transform="translate(-20, 10) scale(.6) rotate({-rnd*130})" fill="#679133">
    <Leaf2 />
  </g>
  <g transform="translate(0, 0) scale(.3) rotate({-rnd*450})" fill="#74a339">
    <Leaf1 />
  </g>
  <g transform="translate(0, 10) scale(0.25) rotate({rnd*360})" fill="#74a339">
    <Leaf2 />
  </g>
  <g transform="translate(-5, -5) scale(0.25) rotate({-rnd*270})" fill="#6C9D2D">
    <Leaf1 />
  </g>
  <polygon x="0" y="0" fill="#6C9D2D" points="{generateLeafPath(scale, depth.toString())}" />

</g>
{/if}

