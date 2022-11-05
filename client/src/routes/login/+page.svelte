<script lang="ts">
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
  }
</script>

<form on:submit={handleLogin}>
  <label>
    Username
    <input type="text" placeholder="username" bind:value={username} />
  </label>

  <label>
    Password
    <input type="password" bind:value={password} />
  </label>

  <input type="submit" value="Log In" />
</form>