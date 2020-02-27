import { Universe, create_buffer } from "wasm-game-of-life";

const width = 64 * 10;
const height = 64 * 10;

// Construct the universe, and get its width and height.
const universe = Universe.new(width, height);

// Give the canvas room for all of our cells and a 1px border
// around each of them.
const canvas = document.getElementById("game-of-life-canvas");
canvas.height = height;
canvas.width = width;

const ctx = canvas.getContext('2d');
ctx.imageSmoothingEnabled = false;

const buffer = new Uint8ClampedArray(width * height * 4);

const imageData = new ImageData(buffer, width, height);

const renderLoop = () => {
  universe.tick();
  universe.render_into(buffer);
  ctx.putImageData(imageData, 0, 0);

  requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);
