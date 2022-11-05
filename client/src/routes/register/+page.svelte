<script lang="ts">
	import { goto } from "$app/navigation";
	import LoginRegister from "$lib/components/LoginRegister.svelte";
  import { userStore } from "$lib/stores/user";

  let username = "";
  let password = "";

  const handleRegister = async (e: any) => {
    e.preventDefault();

    const data = await fetch("https://bonsai-health.shuttleapp.rs/register", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ username, password }),
    }).then((res) => res.json());

    $userStore = await data.token;

    window.localStorage.setItem("token", $userStore);
    window.localStorage.setItem("username", data.username);

    goto("/");
  }
</script>

<svelte:head>
  <title>Register</title>
</svelte:head>

<div class="container">
  <h1>Register</h1>
  
  <LoginRegister 
    handleSubmit={handleRegister} 
    linkHref="/login"
    linkText="Already have an account? Login here!"
    buttonText="Register"
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