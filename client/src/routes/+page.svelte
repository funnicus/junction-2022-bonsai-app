<script lang="ts">
	import Tree from "$lib/components/Tree.svelte";
	import type { Data } from "$lib/dataSchema";
	import { createTreeStore } from "$lib/stores/tree";

  let tree = createTreeStore()

  let angle = 0
  let length = 50

  $:console.log(tree)

</script>
<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>

<a href="/bonsai">To 🅱️ONSAI</a>

<div>
  <Tree
    nodes={$tree.nodes}
    on:select={e => tree.setSelectedNode(e.detail)}
    selectedNode={$tree.selectedNode}
    {angle}
    {length}
  />
</div>

<label>
  angle: {angle} deg
  <input type="range" min="-45" max="45" bind:value={angle} />
</label>

<label>
  length: {length}
  <input type="range" min="20" max="75" bind:value={length} />
</label>

<button on:click={tree.addLeaf}>add leaf</button>
<button on:click={() => tree.addExtension(angle, length)}>add extension</button>

<style>
  label {
    display: flex;
    flex-direction: column;
  }
</style>