# Conway's Snowflake

[![Build Status](https://travis-ci.com/asafigan/conways-snowflake.svg?branch=master)](https://travis-ci.com/asafigan/conways-snowflake)

## Live Demo

Check out the [live demo](https://asafigan.github.io/conways-snowflake/). Refresh the page to generate a new world.

## About

Generates symmetrical [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) universes with [torus typologies](https://www.conwaylife.com/wiki/Torus).

Made using [Rust 🦀 and WebAssembly 🕸](https://rustwasm.github.io/docs/book/) and modified to optimize rendering performance.

### Symmetry

Universes are horizontally and vertically symmetrical throughout the simulation.

### Rendering

The largest optimization was to render using WebAssembly.

Each cell is represented as a single pixel. Black is an alive cell and white is a dead cell.

The universe is rendered it into an [Uint8ClampedArray](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Uint8ClampedArray) and interpreted as [ImageData](https://developer.mozilla.org/en-US/docs/Web/API/ImageData). This ImageData is then displayed onto a canvas using [putImageData()](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/putImageData).

## Build

### Requirements
- [`rust-toolchain`](https://www.rust-lang.org/tools/install)
- [`wasm-pack`](https://rustwasm.github.io/wasm-pack/installer/)
- [`npm`](https://www.npmjs.com/get-npm)

### Steps
- Run `wasm-pack build` inside project dictionary
- Run `npm install` inside [`www`](https://github.com/sn99/wasm-template-rust/tree/master/www) folder
- Finally run `npm run start` inside [`www`](https://github.com/sn99/wasm-template-rust/tree/master/www) and visit http://localhost:8080 to see the results

## License

Licensed under either of these:

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   https://opensource.org/licenses/MIT)
