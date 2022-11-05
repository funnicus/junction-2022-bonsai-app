<script lang="ts">
	import Button from "$lib/components/Button.svelte";
  
  export let handleSubmit: any;
  export let linkText: string;
  export let buttonText: string;
  export let linkHref: string;

  export let username = "";
  export let password = "";
</script>

<form on:submit={handleSubmit}>
  <label>
    Username
    <input type="text" placeholder="username" bind:value={username} />
  </label>

  <label>
    Password
    <input type="password" placeholder="password" bind:value={password} />
  </label>

  <div class="form-footer">
    <Button type="submit">
      {buttonText}
    </Button>
  
    <a href={linkHref}>{linkText}</a>
  </div>
</form>

<style>
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