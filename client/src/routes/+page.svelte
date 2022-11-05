<script lang="ts">
	import Tree from "$lib/components/Tree.svelte";
	import { createTreeStore } from "$lib/stores/tree";
  import Menu from "$lib/components/Menu.svelte";
	import { onMount, setContext } from "svelte";
	import type { Data } from "$lib/dataSchema";
	import { createMenuStore } from "$lib/stores/menu";

  export let tree: Data[];

  let menuState = createMenuStore();

  const treeStore = createTreeStore()

  const handleKeyDown = (event: KeyboardEvent) => {
    if(event.code === "Tab") {
      event.preventDefault()
      treeStore.toggleLeaves(false)
    }
  }

  const handleKeyUp = (event: KeyboardEvent) => {
    if(event.code === "Tab") {
      event.preventDefault()
      treeStore.toggleLeaves(true)
    }
  }

  setContext('menuState', menuState);
  setContext('tree', treeStore);


  onMount(() => {
    fetch("https://bonsai-health.shuttleapp.rs/").then((data) => console.log(data))
  })

</script>

<svelte:window on:keydown={handleKeyDown} on:keyup={handleKeyUp} /> 

<div class="tree-view">
  <Tree store={treeStore} />
</div>

<div style="display: flex; justify-content: center;">
    <Menu />
</div>

<label>
  angle: {$treeStore.previewAngle} deg
  <input type="range" min="-45" max="45" bind:value={$treeStore.previewAngle} />
</label>

<label>
  length: {$treeStore.previewLength}
  <input type="range" min="20" max="75" bind:value={$treeStore.previewLength} />
</label>

<button on:click={() => $treeStore.selectedNode && treeStore.removeNode($treeStore.selectedNode)}>delete node</button>
<button on:click={treeStore.addLeaf}>add leaf</button>
<button on:click={() => treeStore.addExtension($treeStore.previewAngle, $treeStore.previewLength)}>
  add extension
</button>

<style>
  label {
    display: flex;
    flex-direction: column;
  }

  .tree-view {
    display: flex;
  }
</style>


