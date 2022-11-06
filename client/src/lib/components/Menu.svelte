<script lang="ts">
    import type { Task } from "$lib/menuTypes";
    import ArrowLeft from "$lib/icons/ArrowLeft.svelte";
	import CloseIcon from "$lib/icons/CloseIcon.svelte";
	import Check from "$lib/icons/Check.svelte";
	import { getContext, onMount } from "svelte";
	import type { Writable } from "svelte/store";
	import type { TreeStore } from "$lib/stores/tree";
	import { userStore } from "$lib/stores/user";
	import { fly } from "svelte/transition";
	import Button from "./Button.svelte";
	import { shuffleArray } from "$lib/utils/array";

    const treeState = getContext<TreeStore>('tree');
    const state = getContext<Writable<{state: number; isLeaf: number}>>('menuState');
    const tree = getContext<TreeStore>('tree');
    let tasks: Task[] = [];
    let selectedOption: Task;

    onMount( async () => {
        const fetchedTasks = await fetch("https://bonsai-health.shuttleapp.rs/tasks/get_tasks").then(d => d.json());
        tasks = shuffleArray(fetchedTasks)
    });

    function handleOptionSelection(selection: Task){
        selectedOption = selection;
        if($state.state < 0){
            $state.state = 1;
        }else{
            $state.state++;
        }
    }

    function handleBackClick(){
        if($state.state === 0){
            treeState.setSelectedNode(null)

        }
        $state.state--;
    }

    function handleOptionCompletion(){
        $state.state++;
    }

    function resetState(){
        // Menu is hidden
        $state.state = -1;
        if($state.isLeaf){
            tree.addLeaf();
        } else {
            tree.addExtension($tree.previewAngle, $tree.previewLength);
        }    

        tree.setSelectedNode(null);
        $state.isLeaf = 0;

        fetch("https://bonsai-health.shuttleapp.rs/user/edit_tree", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
                "Authorization": "Bearer " + $userStore
            },
            body: JSON.stringify({
                nodes: $tree.nodes,
            }),
        })
    }
</script>

{#if $state.state >= 0}
<div class="menuContainer" transition:fly={{y: 100, duration: 200, opacity: 0}}>
    {#if $state.state === 0}
        <!-- svelte-ignore a11y-click-events-have-key-events -->
        <div class="backIcon" on:click={handleBackClick}>
            <CloseIcon />
        </div>

        <div class="title">
            <b>How do you want to grow your Bonsai?</b>
        </div>
        <div style="display: flex; flex-direction: column; justify-content: space-evenly;">
            <div style="display: flex; margin-top: .5rem; margin-bottom: .5rem;">
                <label>
                    <input type=radio bind:group={$state.isLeaf} value={0} /> Branch
                </label>
    
                <label style="margin-left: 10px">
                    <input type=radio bind:group={$state.isLeaf} value={1} /> Leaf
                </label>
            </div>

            {#if !$state.isLeaf}
            <label class="slider">
                <div>Angle: <b>{$tree.previewAngle}</b>°</div>
                <input type="range" min="-45" max="45" bind:value={$tree.previewAngle} />
            </label>
    
            <label class="slider">
                <div>Length: <b>{$tree.previewLength}</b></div>
                <input type="range" min="20" max="75" bind:value={$tree.previewLength} />
            </label>
            {/if}
        </div>

        <Button type="button" style="margin-top: auto" onClick={handleOptionCompletion}>
            <b>Confirm</b>
        </Button>
    {/if}  

    {#if $state.state !== 0 && $state.state !== 3}
        <!-- svelte-ignore a11y-click-events-have-key-events -->
        <div class="backIcon" on:click={handleBackClick}>
            <ArrowLeft />
        </div>
    {/if}

    {#if $state.state === 1}
        <div class="title">
            <b>What do you want to do?<b>
        </div>

        {#each tasks.slice(0,4) as task }
            <!-- svelte-ignore a11y-click-events-have-key-events -->
            <div class="menuItem" on:click={() => handleOptionSelection(task)}>{task.title}</div>
        {/each}
    {/if}    
    
    {#if $state.state === 2}
        <div class="title">
            <b>{selectedOption.title}</b>
        </div>

        <div class="menuItemDescription">{selectedOption.description}</div>

        <Button type="button" style="margin-top: auto;" onClick={handleOptionCompletion}>Start</Button>
    {/if}

    {#if $state.state === 3}
        <div class="title-2">
            <b>
                <i>{selectedOption.title}</i> started!
            </b>
        </div>

        <!-- svelte-ignore a11y-click-events-have-key-events -->
        <Button type="button" onClick={resetState}>
            Mark as done <Check />
        </Button>
    {/if}
</div>
{/if}

<style>
  .menuContainer {
    color: rgb(161, 80, 34);
    width: 400px;
    height: 350px;
    max-width: calc(100vw - 2rem);
    padding: 16px;
    display: flex;
    flex-flow: column;
    border-radius: 20px;
    box-shadow: 0px 2px 10px 0px grey;
    background: #ffeadf;
}

  .title {
    align-self: center;
    margin-top: 10px;
    margin-bottom: 10px;
    height: fit-content;
    font-size: 1.2em;
  }

  .title-2 {
    align-self: center;
    margin: auto;
    height: fit-content;
    font-size: 1.2em;
  }

  .menuItem {
    margin: 10px;
    padding: 10px;
    border-radius: 5px;
    transition: 0.3s;
    background: rgba(161, 80, 34, .15);
  }

  .menuItem:hover {
    cursor: pointer;
    transform: scale(1.05);

  }

  .menuItemDescription {
    margin: 20px;
  }

  .backIcon {
    display: flex;
    align-self: start;
    transition: 0.6s;
  }
  
  .backIcon:hover {
    cursor: pointer;
    transform: scale(1.25);
  }

  .slider {
    display: flex;
    flex-direction: column;
    align-content: space-around;
    margin: 10px 0;
    width: 100%;
    align-self: center;
    background: rgba(161, 80, 34, .15);
    border-radius: 5px;
    padding: 10px;
  }
</style>