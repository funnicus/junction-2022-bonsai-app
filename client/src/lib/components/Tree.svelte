<script lang="ts">
	import type { TreeStore } from "$lib/stores/tree";
	import TreeNode from "./TreeNode.svelte";
	import Vase from "./Vase.svelte";
  
  export let store: TreeStore;

  const INITIAL_WIDTH = 15;
</script>

<svg viewBox="0 0 500 500" width="500">
  <g transform="translate(250 400) rotate(180)">
    {#each $store.nodes as n (n.time)}
      <TreeNode node={n} depth={1} width={INITIAL_WIDTH} />
    {/each}
  </g>

  <g transform="translate({250 - 114 / 2} 400) scale(0.5)">
    <Vase />
  </g>
</svg>

<style>
  svg {
    overflow: visible;
    margin: auto;
    width: 100%;
  }
</style>