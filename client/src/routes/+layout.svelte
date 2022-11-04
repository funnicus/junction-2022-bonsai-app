
<nav>
  <a href="/">Home</a>
  <a href="/bonsai">Bonsai</a>
  <a href="/settings">Settings</a>
</nav>

<main>
  <slot></slot>
</main>


<style lang="scss" global>
  :root {
    background: #FFE2D1;
    height: 100%;
  }

  body {
    margin: 0;
  }

  .view {
    height: 100vh;
    align-items: center;
    margin: 0;
    display: flex;
    flex-direction: column;

    main {
      flex: 1 1 75%;
    }
  }


</style>
