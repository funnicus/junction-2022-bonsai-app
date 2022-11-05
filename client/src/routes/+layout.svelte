<script>
	import { onMount } from "svelte";
	import { goto } from "$app/navigation";

	import Settings from "$lib/icons/Settings.svelte";
  import QuestionMark from "$lib/icons/QuestionMark.svelte";
  
  import { userStore } from "$lib/stores/user";

  onMount(() => {
    $userStore = window.localStorage.getItem("token") || "";
  });

</script>


<nav>
  <div>
    <a href="/settings"><Settings /></a>
    <a href="questionnaire"><QuestionMark /></a>
  </div>

  {#if $userStore}
    <span>Logged in as <b>{"user"}</b></span>
  {/if}

  <div>
    {#if $userStore}
      <button class="logging-btn" on:click={() => {
          window.localStorage.removeItem("token")
          window.localStorage.removeItem("tree")
          $userStore = ""
          goto("/login")
        }
      }>Logout</button>
    {:else}
      <a href="/login">Login</a>
    {/if}
  </div>
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

    position: relative;
    z-index: 3;

    display: flex;
    justify-content: space-between;
    a {
    color: #A15022;
  }
  }

  .logging-btn {
    background: none;
    border: none;
    color: var(--brown);
    font-size: 1.2rem;
    font-weight: 700;
    cursor: pointer;
  }

  .logging-btn:hover {
    text-decoration: underline;
  }


  body {
    margin: 0;
    height: 100%;
  }
</style>
