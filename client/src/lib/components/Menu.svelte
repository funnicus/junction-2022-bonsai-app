<script lang="ts">
    import type { MenuContent, MenuItem } from "$lib/menuTypes";
    import ArrowLeft from "$lib/assets/ArrowLeft.svelte";

    let state = 0;
    let selectedOption: MenuItem;
    const defaultTitle = "What would you like to do?"

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
        console.log(`Switching state from ${state} to ${state + 1}`)
        if(state < 0){
            state = 1;
        }else{
            state++;
        }
    }

    function handleBackClick(){
        if(state === 0){
            console.log("Closing menu")
        }
        console.log(`Switching state from ${state} to ${state - 1}`)
        state--;
    }
</script>

{#if state >= 0}
<div class="menuContainer">
        <!-- svelte-ignore a11y-click-events-have-key-events -->
        <div class="backIcon" on:click={handleBackClick}>
            <ArrowLeft type="right-arrow"  />
        </div>
        

        {#if state === 0}
            <div class="title">
                <b>{defaultTitle}</b>
            </div>

            {#each menuContent.options as m }
                <!-- svelte-ignore a11y-click-events-have-key-events -->
                <div class="menuItem" on:click={() => handleOptionSelection(m)}>{m.title}</div>
            {/each}
        
        {:else}
            <div class="title">
                {selectedOption.title}
            </div>

            <div class="menuItemDescription">{selectedOption.description}</div>

        {/if}
</div>
{/if}



<style>
  .menuContainer {
    padding: 15px;
    display: flex;
    flex-flow: column;
    align-items: center;
    border-radius: 20px;
    /* background: linear-gradient(to bottom, rgba(0,0,0,0), #FFE2D1 25%); */
    box-shadow: 0px 2px 10px 0px grey;
}

  .title {
    margin-top: 10px;
    margin-bottom: 10px;
    height: fit-content;
    font-size: 1.2em;
  }

  .menuItem {
    margin: 20px;
    /* border: 1px black solid; */
    /* background: rgba(255,255,255,.6); */
    border-radius: 5px;
    animation-duration: 2s;
    animation-iteration-count: infinite;
  }

  .menuItem:hover {
    cursor: pointer;
    animation-name: bounce;
    animation-timing-function: ease;
  }

  .backIcon {
    display: flex;
    align-self: start;
  }
  
  .backIcon:hover {
    cursor: pointer;
    animation-name: bounce;
    animation-timing-function: ease;
  }

  @keyframes bounce {
    0%   { transform: scale(1); }
    25%  { transform: scale(1.1); }
    100% { transform: scale(1); }
  }
</style>