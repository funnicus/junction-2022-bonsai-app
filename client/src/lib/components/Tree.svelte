<script lang="ts">
	import type { Data } from "../dataSchema";
	import TreeNode from "./TreeNode.svelte";
	import Vase from "./Vase.svelte";
  
  export let nodes: Data[]
  export let selectedNode: Data | null

  // used for preview
  export let angle: number;
  export let length: number;
</script>

<svg viewBox="0 0 500 500" width="500">
  <g transform="translate(250 400) rotate(180)">
    {#each nodes as n (n.time)}
      <TreeNode node={n} depth={1} width={15} {selectedNode} on:select {angle} {length} />
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
  }
</style>