<script lang="ts">
	import Tree from "$lib/components/Tree.svelte";
	import { createTreeStore } from "$lib/stores/tree";
  import Menu from "$lib/components/Menu.svelte";
	import { onMount, setContext } from "svelte";

  let tree = createTreeStore()

  setContext("tree", tree);

  let angle = 0
  let length = 50

  //$:console.log(tree)

  onMount(() => {
    fetch("https://bonsai-health.shuttleapp.rs/").then((data) => console.log(data))
  })

  $:console.log($tree.nodes)

  const handleKeyDown = (event: KeyboardEvent) => {
    if(event.code === "Tab") {
      event.preventDefault()
      tree.toggleLeaves(false)
    }
  }

  const handleKeyUp = (event: KeyboardEvent) => {
    if(event.code === "Tab") {
      event.preventDefault()
      tree.toggleLeaves(true)
    }
  }

</script>

<svelte:window on:keydown={handleKeyDown} on:keyup={handleKeyUp} /> 

<div class="tree-view">
  <Tree
    nodes={$tree.nodes}
    on:select={e => tree.setSelectedNode(e.detail)}
    selectedNode={$tree.selectedNode}
    {angle}
    {length}
  />
</div>

<div style="display: flex; justify-content: center;">
    <Menu />
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

  .tree-view {
    display: flex;
  }
</style>


