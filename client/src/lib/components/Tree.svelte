<script lang="ts">
	import type { Data } from "../dataSchema";
	import TreeNode from "./TreeNode.svelte";
  export let nodes: Data[]
</script>

<svg>

  <g transform="translate(100 100) rotate(180)">

    {#each nodes as n (n.time)}
      <TreeNode node={n} />
    {/each}
  </g>
</svg>


<style>
  svg {
    overflow: visible;
    border: 1px solid red;
  }
</style>