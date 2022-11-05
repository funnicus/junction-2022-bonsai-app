<script lang="ts">
	import Tree from "$lib/components/Tree.svelte";
	import { createTreeStore } from "$lib/stores/tree";
  import Menu from "$lib/components/Menu.svelte";
	import { onMount, setContext } from "svelte";
	import { createMenuStore } from "$lib/stores/menu";
	import { goto } from "$app/navigation";
	import ToolTips from "$lib/components/ToolTips.svelte";

  const menuState = createMenuStore();

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

  onMount(async () => {
    const user = await fetch("https://bonsai-health.shuttleapp.rs/user/data", {
      method: "GET",
      headers: {
        "Content-Type": "application/json",
        "Authorization": "Bearer " + window.localStorage.getItem("token")
      },
    }).then((res) => res.json()).catch(() => goto("/"));

    if(user.data) {
      treeStore.setNodes(user.data.nodes)
    }
    
  })
</script>

<svelte:window on:keydown={handleKeyDown} on:keyup={handleKeyUp} /> 

<svelte:head>
  <title>The Bonsai</title>
</svelte:head>

<ToolTips />

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
    position: absolute;
    top: 0;
    left: 0;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    width: 100vw;
    height: 100vh;
    overflow: hidden;
  }

  .tree-view {
    transform: translateY(150px);
    transition: transform 500ms;
  }

  .page.menuOpen .tree-view {
    transform: scale(1) translateX(-100px) translateY(150px);
  }

  .menu {
    position: absolute;
    display: flex;
    bottom: 2rem;
    right: 2rem;
    justify-content: center;
    opacity: 0;
    transition: opacity 200ms;
  }
  
  .menu.visible {
    opacity: .8;
  }

  @media screen and (max-width: 700px) {
    .page.menuOpen .tree-view {
      transform: scale(1) translateY(-80px);
    }


    .menu {
      right: unset;
    }
  }
</style>


