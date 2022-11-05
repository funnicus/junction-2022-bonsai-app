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
    }).then((res) => res.json());

    $userStore = await data.token;

    window.localStorage.setItem("token", $userStore);
    window.localStorage.setItem("username", data.username);

    goto("/");
  }
</script>

<LoginRegister 
  handleSubmit={handleLogin} 
  linkHref={"/register"} 
  linkText={"Don't have an account yet? Register here"} 
  buttonText={"Login"} 
  bind:username={username}
  bind:password={password}
/>

<style>
  form {
    max-width: 40rem;
    margin: 0 auto;
    margin-top: 5rem;
  
  }

  .form-footer {
    display: flex;
    justify-content: space-between;
  }

  label {
    display: flex;
    flex-direction: column;
    margin-bottom: 1rem;
  }

  input[type="text"], input[type="password"] {
    padding: .5rem;
    border-radius: .5rem;
    border: 2px solid var(--brown);
  }

</style>