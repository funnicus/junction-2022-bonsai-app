<script lang="ts">
	import { goto } from "$app/navigation";
	import LoginRegister from "$lib/components/LoginRegister.svelte";
  import { userStore } from "$lib/stores/user";

  let username = "";
  let password = "";

  const handleLogin = async (e: any) => {
    e.preventDefault();

    const data = await fetch("https://bonsai-health.shuttleapp.rs/login", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        username,
        password
      }),
    }).then((res) => res.json()).catch(err => alert("Try again!"));

    if(!data) return;

    $userStore = data.token;

    window.localStorage.setItem("token", $userStore);
    window.localStorage.setItem("username", data.username);

    alert("You have successfully logged in!")

    goto("/");
  }
</script>

<svelte:head>
  <title>Log In</title>
</svelte:head>

<div class="container">
  <h1>Log In</h1>
  
  <LoginRegister 
    handleSubmit={handleLogin} 
    linkHref="/register"
    linkText="Don't have an account yet? Register here"
    buttonText="Login"
    bind:username={username}
    bind:password={password}
  />
</div>

<style>
  .container {
    max-width: 36rem;
    width: calc(100% - 2rem);
    margin: 0 auto;
  }
</style>