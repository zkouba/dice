# Dice Roller — web frontend

A browser UI for the `dice` Rust engine. The rolling logic is compiled to
WebAssembly from the same crate, so the engine stays the single source of truth
shared with the native REPL.

## Prerequisites

- Rust toolchain + the wasm target: `rustup target add wasm32-unknown-unknown`
- [`wasm-pack`](https://rustwasm.github.io/wasm-pack/): `cargo install wasm-pack`
- Node.js + npm

## Develop

```bash
cd web
npm install
npm run dev      # rebuilds the wasm package, then starts Vite on :5173
```

## Build a static bundle

```bash
cd web
npm run build    # wasm + typecheck + Vite build -> web/dist
npm run preview  # serve the built bundle locally
```

The contents of `web/dist` are fully static — host them on any static host
(GitHub Pages, Netlify, …) or open them through any local web server.

## How it fits together

- `npm run wasm` runs `wasm-pack` against the crate with
  `--no-default-features --features wasm`, emitting an ES module into `web/pkg`
  (gitignored — regenerated on every build).
- [`src/dice.ts`](src/dice.ts) is the typed wrapper over that module; the
  JSON-string boundary it talks to is defined in `../src/wasm_api.rs`.
- [`src/sets.ts`](src/sets.ts) persists saved dice sets to `localStorage`.
- [`src/animation.ts`](src/animation.ts) plays the roll animation, which settles
  on the value the engine already returned.
