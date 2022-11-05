<script lang="ts">
	import Tree from "$lib/components/Tree.svelte";
	import type { Data } from "$lib/dataSchema";

  let testData: Data[] = [
    {
      type:'extension',
      length: 20,
      angle: 5,
      children: [],
      taskId: "",
      time: Date.now()
    }
  ]
  let selectedNode: Data | null = null

  let angle = 0
  let length = 50

  const addLeaf = () => {
    const item: Data =  {
      type:'leaf',
      children: [],
      taskId: "123981628362",
      time: Date.now()
    }

    if (!selectedNode) return

    selectedNode.children.push(item)
    selectedNode = null
    testData = testData // tell svelte to update tree
  }

  const addExtension = () => {
    const item: Data =  {
      type:'extension',
      length,
      angle,
      children: [],
      taskId: "123981628362",
      time: Date.now()
    }

    if (!selectedNode) return

    selectedNode?.children.push(item)
    selectedNode = null
    testData = testData // tell svelte to update tree
  }

  $:console.log(testData)

</script>
<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>

<a href="/bonsai">To 🅱️ONSAI</a>

<div>
  <Tree
    nodes={testData}
    on:select={e => selectedNode = e.detail}
    {selectedNode}
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

<button on:click={addLeaf}>add leaf</button>
<button on:click={addExtension}>add extension</button>

<style>
  label {
    display: flex;
    flex-direction: column;
  }
</style>