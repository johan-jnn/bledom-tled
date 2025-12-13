<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  const { children } = $props();
</script>

{#await invoke("device_init", { force: false })}
  <div class="container">
    <h1>Fetching device...</h1>
    <p>Please wait while we're connecting to the led strip...</p>
    <div aria-busy="true"></div>
  </div>
{:then}
  {@render children()}
{:catch err}
  {console.error(err)}
  <div class="container">
    <h1>Oups...</h1>
    <p>It seems that there is no device around you.</p>
    <h2>Did you made all the following steps ?</h2>
    <ol>
      <li>Turn on your bluetooth</li>
      <li>Make sure your device is powered</li>
      <li>Make sure your device is not too far from your computer</li>
    </ol>

    <button type="button" onclick={() => location.reload()}>Try again</button>
    <a href="/">Back to home</a>
  </div>
{/await}

<style lang="scss">
  .container {
    margin-top: 2em;
  }
</style>
