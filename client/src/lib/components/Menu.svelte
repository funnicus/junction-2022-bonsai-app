<script lang="ts">
    import type { MenuContent, MenuItem } from "$lib/menuTypes";
    import ArrowLeft from "$lib/icons/ArrowLeft.svelte";
	import CloseIcon from "$lib/icons/CloseIcon.svelte";
	import Check from "$lib/icons/Check.svelte";
	import { getContext } from "svelte";
	import type { Writable } from "svelte/store";

    const state = getContext<Writable<{state: number}>>('menuState');
    let selectedOption: MenuItem;

    const menuContent: MenuContent = {
        title: "What would you like to do?",
        options: [{
            title: "Brush your teeth",
            description: "Do something other than just laying in bed - keeping hygiene levels high improves both mental and physical health"
        },{
            title: "Take out the trash",
            description: "Do something other than just laying in bed - keeping the house clean improves both mental and physical health"
        },{
            title: "Call a friend",
            description: "Do something other than just laying in bed - calling a friend is always fun!"
        }]
    }

    function handleOptionSelection(selection: MenuItem){
        selectedOption = selection;
        console.log(`Switching state from ${$state.state} to ${$state.state + 1}`)
        if($state.state < 0){
            $state.state = 1;
        }else{
            $state.state++;
        }
    }

    function handleBackClick(){
        if($state.state === 0){
            console.log("Closing menu")
        }
        console.log(`Switching state from ${$state.state} to ${$state.state - 1}`)
        $state.state--;
    }

    function handleOptionCompletion(){
        console.log(`Switching state from ${$state.state} to ${$state.state + 1}`)
        $state.state++;
    }

    function resetState(){
        // Menu is hidden
        $state.state = -1;
    }
</script>

{#if $state.state >= 0}
<div class="menuContainer">

    {#if $state.state !== 0 && $state.state !== 2}
        <!-- svelte-ignore a11y-click-events-have-key-events -->
        <div class="backIcon" on:click={handleBackClick}>
            <ArrowLeft />
        </div>
    {/if}

    {#if $state.state === 0}
        <!-- svelte-ignore a11y-click-events-have-key-events -->
        <div class="backIcon" on:click={handleBackClick}>
            <CloseIcon />
        </div>

        <div class="title">
            <b>{menuContent.title}</b>
        </div>

        {#each menuContent.options as m }
            <!-- svelte-ignore a11y-click-events-have-key-events -->
            <div class="menuItem" on:click={() => handleOptionSelection(m)}>{m.title}</div>
        {/each}
    {/if}    
    
    {#if $state.state === 1}
        <div class="title">
            <b>{selectedOption.title}</b>
        </div>

        <div class="menuItemDescription">{selectedOption.description}</div>

        <button class="button" on:click={handleOptionCompletion}> <b> Mark as complete </b> </button>
    {/if}

    {#if $state.state === 2}
        <div class="title">
            <b>
                <i>{selectedOption.title}</i> complete!
            </b>
        </div>

        <!-- svelte-ignore a11y-click-events-have-key-events -->
        <div class="checkMark" on:click={resetState}>
            <Check />
        </div>
    {/if}
</div>
{/if}



<style>
    @import url('https://fonts.googleapis.com/css2?family=Inter:wght@200&display=swap');

  .menuContainer {
    border: 2px solid rgb(161, 80, 34);
    color: rgb(161, 80, 34);
    width: 400px;
    min-height: 300px;
    font-family: 'Inter', sans-serif;
    padding: 15px;
    display: flex;
    flex-flow: column;
    border-radius: 20px;
    box-shadow: 0px 2px 10px 0px grey;
}

  .title {
    align-self: center;
    margin-top: 10px;
    margin-bottom: 10px;
    height: fit-content;
    font-size: 1.2em;
  }

  .menuItem {
    margin: 20px;
    border-radius: 5px;
    transition: 0.3s;
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

  .button {
    margin-top: auto;
    color: rgb(161, 80, 34);
    width: fit-content;
    padding: 10px;
    align-self: center;
    border-radius: 5px;
    border: none;
    background: rgba(161, 80, 34, .15);
    transition: 0.3s;
  }
  button:hover {
    cursor: pointer;
    transform: scale(1.15);
  }

  .checkMark {
    margin-top: auto;
    color: rgb(161, 80, 34);
    width: fit-content;
    padding: 10px;
    align-self: center;
    justify-self: end;
    border-radius: 5px;
    border: none;
    background: rgba(161, 80, 34, .15);
    transition: 0.3s;
  }

  .checkMark:hover {
    cursor: pointer;
    transform: scale(1.15);
  }

</style>