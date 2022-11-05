<script lang="ts">
  export let text: string;
  export let type: string | undefined;
</script>

<button type={type} class="button"> <b>{text}</b> </button>

<style>
  .button {
    color: rgb(161, 80, 34);
    width: fit-content;
    padding: 10px;
    align-self: center;
    border-radius: 5px;
    border: none;
    background: rgba(161, 80, 34, .15);
    transition: 0.3s;
  }

  button:hover {
    cursor: pointer;
    background: rgba(161, 80, 34, .15);
    transform: scale(1.15);
  }
</style>