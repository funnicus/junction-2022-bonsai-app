<script lang="ts">
	import type { TreeStore } from "$lib/stores/tree";
	import { degToRad } from "$lib/utils/number";
	import { createEventDispatcher, getContext } from "svelte";
	import { scale } from "svelte/transition";
	import type { Data } from "../dataSchema";
	import Leaf from "./Leaf.svelte";

  export let node: Data
  export let depth: number
  export let width: number;

  const tree = getContext<TreeStore>('tree')

  $: selected = $tree.selectedNode === node

  const x2 = -Math.sin(degToRad(node?.angle)) * (node?.length ?? 0)
  const y2 = Math.cos(degToRad(node?.angle)) * (node?.length ?? 0)

  const currentWidth = Math.max(width - (depth*1.5), 3);
</script>

<g
  in:scale={{
    start: 0,
    duration: 1500,
    opacity: 1,
    delay: Math.min((depth) * 25, 50)
  }}>
  {#if node.type === "extension"}
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <polygon
      on:click={() => tree.setSelectedNode(selected ? null : node)}
      class:selected
      points="
        {-currentWidth/2},0
        {-((currentWidth/2)-0.5)},{node.length}
        {(currentWidth/2)-0.5},{node.length}
        {currentWidth/2},0
        0, -5"
      fill="#834A4A"
      transform="rotate({node.angle})"
      stroke-linejoin="round"
    />
    
    <g transform="translate({x2} {y2}) rotate({node.angle})">
      {#if selected}
        <line
          x1={0}
          y1={0}
          x2={-Math.sin(degToRad($tree.previewAngle)) * (($tree.previewLength-4) ?? 0)}
          y2={Math.cos(degToRad($tree.previewAngle)) * (($tree.previewLength-4) ?? 0)}
          stroke-dasharray="4"
          stroke="blue"
          stroke-width={3}
        />

        <circle
          cx={-Math.sin(degToRad($tree.previewAngle)) * (($tree.previewLength-4) ?? 0)}
          cy={Math.cos(degToRad($tree.previewAngle)) * (($tree.previewLength-4) ?? 0)}
          r="5"
          stroke="blue"
          fill="transparent"
          stroke-width={2}
        />

      {/if}

      {#each node.children as child}
        <svelte:self
          node={child}
          depth={depth + 1}
          {width}
        />
      {/each}
    </g>
  {:else}
    <Leaf depth={depth + 1} {width} />
  {/if}
</g>


<style>
  polygon {
    stroke: transparent;
    stroke-width: 0;
    transition: stroke 200ms, stroke-width 200ms;
  }

  polygon.selected {
    stroke: blue;
    stroke-width: 2;

  }
</style>