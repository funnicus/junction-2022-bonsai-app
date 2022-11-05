<script lang="ts">
	import { userStore } from "$lib/stores/user";
</script>

<div class="tooltips">
  <ul>
    <li>Hold TAB to hide leaves</li>
  </ul>


  <p>Click on the tree to begin</p>

  {#if !$userStore}
    <p>Login to save your progress</p>
  {/if}
</div>

<style lang="scss">
  .tooltips {
    position: absolute;
    width: fit-content;
    top: 1.25rem;
    left: 4rem;
    z-index: 1;

    font-size: 14px;

    &> ul {
      list-style: none;
      margin: 0;
      padding: 0;

    }
  }
</style>