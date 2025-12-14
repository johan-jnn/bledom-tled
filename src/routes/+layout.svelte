<script lang="ts">
  import { page } from "$app/state";
  import { House } from "@lucide/svelte";
  import { app } from "@tauri-apps/api";

  const { children } = $props();
</script>

<div>
  <header class="container pico">
    {#if page.route.id !== "/"}
      <a href="/">
        <House />
        Homepage
      </a>
    {/if}
  </header>
  <main>
    {@render children()}
  </main>

  <footer class="pico">
    <p>
      {#await app.getName() then name}
        {name}
      {/await}
      by <a href="https://johan-janin.com">Johan JANIN</a>
    </p>
  </footer>
</div>

<style lang="scss">
  div {
    display: grid;
    grid-template-rows: auto 1fr auto;
    height: 100svh;
    width: 100%;
  }

  header {
    display: flex;
    align-items: center;
    justify-content: space-between;

    &:not(:empty) {
      padding: {
        top: 0.25em;
        bottom: 0.25em;
      }
    }

    * {
      margin-bottom: 0;
    }

    > a {
      display: flex;
      align-items: center;
      gap: 0.5em;
    }
  }

  main {
    height: 100%;
    overflow-y: scroll;
  }

  footer {
    padding: 0.25em;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5em;

    > * {
      margin-bottom: 0;
    }
  }
</style>
