<script>
	import Settings from "$lib/icons/Settings.svelte";
</script>


<nav>
  <a href="/settings"><Settings /></a>
</nav>

<main>
  <slot></slot>
</main>


<style lang="scss" global>
  :root {
    background: #FFE2D1;
    height: 100%;
  }

  nav {
    margin: 16px;

    a {
    color: #A15022;
  }
  }



  body {
    margin: 0;
  }

  .view {
    height: 100vh;
    margin: 0;
    display: flex;
    flex-direction: column;

    nav {
      height: 25%;
    }
    main {
      height: 75%;
    }
  }


</style>
