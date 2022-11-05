<script>
	import ArrowLeft from "$lib/icons/ArrowLeft.svelte";

</script>
<a href="/"><ArrowLeft /></a>

<slot />

<style>
  a {
    position: absolute;
    left: 1rem;
    color: var(--brown);
  }
</style>