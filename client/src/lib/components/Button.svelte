<script lang="ts">
  export let type: string | undefined;
  export let style = "";
  export let onClick: ((event: Event) => void | Promise<void>) | undefined = undefined;
</script>

<button style={style} type={type} on:click={onClick} class="button">
  <slot />
</button>

<style>
  .button {
    color: rgb(161, 80, 34);
    width: fit-content;
    padding: 10px;
    align-self: center;
    display: flex;
    flex-direction: row;
    align-items: center;
    gap: 1rem;
    border-radius: 5px;
    border: none;
    background: rgba(161, 80, 34, .15);
    transition: 0.3s;
    font-weight: bold;
  }

  .button:hover {
    cursor: pointer;
    background: rgba(161, 80, 34, .15);
    transform: scale(1.15);
  }
</style>
