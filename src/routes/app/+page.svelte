<script lang="ts">
  import { goto, replaceState } from "$app/navigation";
  import { page } from "$app/state";
  import { Power, PowerOff } from "@lucide/svelte";
  import { invoke } from "@tauri-apps/api/core";
  import debounce from "debounce";
  import { TabItem, Tabs } from "flowbite-svelte";
  import { BuiltinEffects } from "../../types/builtin_effect";
  import type { Device } from "../../types/device.js";

  const DEVICE_LOCALSTORAGE_KEY = "device_save";

  // Clearing the "force query if any" and delete the json saved data
  if (page.url.searchParams.has("force")) {
    page.url.searchParams.delete("force");
    window.localStorage.removeItem(DEVICE_LOCALSTORAGE_KEY);

    replaceState(page.url, {});
  }

  const { data } = $props();

  let device = $state({
    _tab: "colors",
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
    if (!(device.is_on && device._tab === "colors")) return;
    const [r, g, b] = device.rgb_color;
    invoke("device_change_only", { r, g, b }).catch(console.error);
  });
  $effect(() => {
    if (!(device.is_on && device._tab === "colors")) return;
    invoke("device_change_only", { a: device.brightness }).catch(console.error);
  });
  $effect(() => {
    if (!(device.is_on && device._tab === "effects")) return;
    if (device.effect !== undefined) {
      invoke("device_set_effect", { effect: device.effect }).catch(
        console.error
      );
    }
  });
  $effect(() => {
    if (!(device.is_on && device._tab === "effects")) return;
    if (device.effect_speed !== undefined) {
      invoke("device_set_effect", { speed: device.effect_speed }).catch(
        console.error
      );
    }
  });

  $effect(() => {
    window.localStorage.setItem(
      DEVICE_LOCALSTORAGE_KEY,
      JSON.stringify(device)
    );
  });
  $inspect(device);
</script>

<form class="container">
  <div class="pico state_manager" class:centerized={!device.is_on}>
    <button
      type="button"
      class="secondary"
      onclick={() => {
        page.url.searchParams.append("force", "");
        goto(page.url);
      }}
    >
      Refetch device
    </button>
    <button type="button" onclick={() => (device.is_on = !device.is_on)}>
      {#if device.is_on}
        <PowerOff />
      {:else}
        <Power />
      {/if}
      Turn {device.is_on ? "off" : "on"}
    </button>
  </div>

  {#if device.is_on}
    <Tabs tabStyle="full" bind:selected={device._tab}>
      <TabItem title="Colorized" key="colors">
        <div class="pico">
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
                () => device.brightness,
                debounce((v) => (device.brightness = v), 25)
              }
              min="0"
              max="255"
              step="1"
            />
          </label>
        </div>
      </TabItem>

      <TabItem title="Effects" key="effects">
        <div class="pico">
          <label for="speed">
            Device's effect speed
            <input
              type="range"
              name="speed"
              id="speed"
              bind:value={
                () => device.effect_speed,
                debounce((v) => (device.effect_speed = v), 25)
              }
            />
          </label>

          <label for="effect">
            Device's effect

            <select name="effect" id="effect" bind:value={device.effect}>
              {#each Object.entries(BuiltinEffects) as [key, value] (key)}
                {#if isNaN(parseInt(key))}
                  <option {value}>{key}</option>
                {/if}
              {/each}
            </select>
          </label>
        </div>
      </TabItem>
    </Tabs>
  {/if}
</form>

<style lang="scss">
  form {
    margin-top: 2em;
  }

  .state_manager {
    display: flex;
    align-items: center;
    justify-content: end;
    gap: 1em;

    &.centerized {
      justify-content: center;
      height: 75svh;
      gap: 0;
      flex-direction: column-reverse;
    }

    > button {
      display: flex;
      align-items: center;
      gap: 1em;
    }
  }
</style>
