<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import debounce from "debounce";
  import type { Device } from "../../types/device.js";

  const { data } = $props();
  const DEVICE_LOCALSTORAGE_KEY = "device_save";

  let device = $state({
    // svelte-ignore state_referenced_locally
    ...((JSON.parse(
      window.localStorage.getItem(DEVICE_LOCALSTORAGE_KEY) ?? "null"
    ) ?? data.device) as Device),
    get hex_color() {
      return "#" + this.rgb_color.map((v) => v.toString(16)).join("");
    },
    set hex_color(value: string) {
      const parsed = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(value);
      if (parsed) {
        this.rgb_color = [
          parseInt(parsed[1], 16),
          parseInt(parsed[2], 16),
          parseInt(parsed[3], 16),
        ];
      }
    },
  });

  $effect(() => {
    invoke("device_toggle", { power: device.is_on }).catch(console.error);
  });
  $effect(() => {
    const [r, g, b] = device.rgb_color;
    invoke("device_change_only", { r, g, b }).catch(console.error);
  });
  $effect(() => {
    invoke("device_change_only", { a: device.brightness }).catch(console.error);
  });

  $effect(() => {
    window.localStorage.setItem(
      DEVICE_LOCALSTORAGE_KEY,
      JSON.stringify(device)
    );
  });
</script>

<form class="container">
  <label for="state">
    Device's state
    <input
      type="checkbox"
      name="power"
      id="power"
      role="switch"
      bind:checked={device.is_on}
    />
  </label>

  <label for="color">
    Device's color
    <input
      type="color"
      name="color"
      id="color"
      bind:value={() => device.hex_color, (v) => (device.hex_color = v)}
    />
  </label>

  <label for="intensity">
    Device's intensity
    <input
      type="range"
      name="intensity"
      id="intensity"
      bind:value={
        () => device.brightness, debounce((v) => (device.brightness = v), 25)
      }
      min="0"
      max="255"
      step="1"
    />
  </label>
</form>

<style lang="scss">
  form {
    margin-top: 2em;
  }
</style>
