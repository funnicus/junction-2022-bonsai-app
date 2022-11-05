<script lang="ts">
	import Tree from "$lib/components/Tree.svelte";
  import data from '$lib/assets/example2.json';

  const d = data as any

</script>
<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>

<a href="/bonsai">To 🅱️ONSAI</a>

<Tree nodes={d} />

