# Bledom-Tled

A small Tauri application to control Bledom Leds from your computer

## Advancement

[x] Leds connexions
[x] Switch ON/OFF
[x] Change color
[x] Change brighness
[x] Save led's status to storage (bledom leds are write-only)
[ ] Switch mode (fades, musics, ...)
[ ] Plugin system (via wasm)

## Installation

For now the application is in developpement only. But if you really want to test it (or to contribute), here is the instruction to build from source :

```sh
git clone https://github.com/johan-jnn/bledom-tled
cd bledom-tled

# If you want, replace 'bun' with your package manager
bun install
bun tauri dev
```
