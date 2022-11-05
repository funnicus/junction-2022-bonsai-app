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

<div class="page" class:menuOpen={$menuState.state > -1}>
  <div class="tree-view">
    <Tree store={treeStore} />
  </div>
  
  <div class="menu" class:visible={$menuState.state > -1}>
      <Menu />
  </div>
</div>

<style>
  .page {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
  }

  .tree-view {
    transform: translateY(150px);
    transition: transform 500ms;
  }

  .page.menuOpen .tree-view {
    transform: scale(1.25) translateY(-70px);
  }

  .menu {
    position: absolute;
    display: flex;
    bottom: 2rem;
    justify-content: center;
    opacity: 0;
    transition: opacity 200ms;
  }
  
  .menu.visible {
    opacity: 1;
  }
</style>


