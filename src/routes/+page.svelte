<script>
  import { app } from "@tauri-apps/api";
</script>

<header class="pico">
  <video
    src="/videos/neon_hearth_background.mp4"
    autoplay
    muted
    loop
    aria-hidden="true"
  ></video>

  <h1>
    Thanks for using
    {#await app.getName() then name}
      {name}
    {/await}
    !
  </h1>
  <a href="/app" role="button">Connect led strips</a>
</header>
<main class="container pico">
  <p>
    This application let you control BLEDOM-like led strips with your computer.
  </p>

  <h2>Huge thanks</h2>
  <p>
    Because I'm a lazy (or a normal) developper, I used a lot of libraries that
    made my life way easier to build this application.
  </p>
  <p>
    If you're interested, you see their pages by clicking on the links bellow :
  </p>

  <ul>
    <li><a href="https://v2.tauri.app/fr/" target="_blank">Tauri 2.0</a></li>
    <li>
      <a href="https://github.com/b1scoito/elk-led-controller" target="_blank">
        ELK-LED-CONTROLLER Rust's crate
      </a>
    </li>
    <li>
      <a href="https://picocss.com" target="_blank"> Pico CSS </a>
    </li>

    <li>
      <a
        href="https://www.vecteezy.com/free-videos/light-strip"
        target="_blank"
      >
        Light Strip Stock Videos by Vecteezy
      </a>
    </li>

    <li>
      <a href="https://flowbite-svelte.com/docs/components/tabs">
        Flowbite's Tabs
      </a>
    </li>
  </ul>
</main>

<style lang="scss">
  header {
    height: 75svh;
    display: grid;
    place-content: center;
    position: relative;
    margin-top: 2em;

    h1 {
      color: white;
    }

    > video {
      position: absolute;
      top: 0;
      left: 0;
      width: 100%;
      height: 100%;
      object-fit: cover;
      z-index: -1;
      pointer-events: none;

      filter: brightness(0.25) blur(10px);
    }
  }

  main {
    margin-top: 2em;
  }
</style>
