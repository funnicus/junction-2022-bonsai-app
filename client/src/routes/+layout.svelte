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
  @import url('https://fonts.googleapis.com/css2?family=Inter:wght@500;700&display=swap');

  :root {
    --bg: #FFE2D1;
    --brown: #A15022;
    --light-brown: #a15122bf;

    background: var(--bg);
    height: 100%;
    font-family: "Inter", sans-serif;

    color: var(--brown);
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
