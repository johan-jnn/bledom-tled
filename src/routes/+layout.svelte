<script lang="ts">
  import { app } from "@tauri-apps/api";

  const { children } = $props();
</script>

<div>
  <header class="container">
    <p>
      {#await app.getName() then name}
        {name}
      {/await}
      by <a href="https://johan-janin.com">Johan JANIN</a>
    </p>
  </header>
  <main>
    {@render children()}
  </main>
</div>

<style lang="scss">
  div {
    display: grid;
    grid-template-rows: auto 1fr;
    height: 100svh;
    width: 100%;
  }

  header {
    display: flex;
    align-items: center;
    justify-content: space-between;

    padding: {
      top: 0.25em;
      bottom: 0.25em;
    }

    * {
      margin-bottom: 0;
    }
  }

  main {
    height: 100%;
    overflow-y: scroll;
  }
</style>
