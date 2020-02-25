import { Universe, Cell, create_buffer } from "wasm-game-of-life";
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";

const CELL_SIZE = 1; // px
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

// Construct the universe, and get its width and height.
const universe = Universe.new();
const width = universe.width();
const height = universe.height();

// Give the canvas room for all of our cells and a 1px border
// around each of them.
const canvas = document.getElementById("game-of-life-canvas");
canvas.height = CELL_SIZE * height;
canvas.width = CELL_SIZE * width;

const ctx = canvas.getContext('2d');

const buffer = create_buffer(CELL_SIZE * width * height * 4);

const imageData = new ImageData(buffer, width, height);

const renderLoop = () => {
  universe.tick();
  universe.render_into(buffer);
  ctx.putImageData(imageData, 0, 0);

  requestAnimationFrame(renderLoop);
};

const clear = () => {
  ctx.fillStyle = ALIVE_COLOR;
  ctx.fillRect(0, 0, CELL_SIZE * width, CELL_SIZE * height);
}

const getIndex = (row, column) => {
  return row * width + column;
};

const drawCells = () => {
  const cellsPtr = universe.cells();
  const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

  ctx.beginPath();
  ctx.fillStyle = DEAD_COLOR;

  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const idx = getIndex(row, col);
      if (cells[idx] === Cell.Alive) {
        continue;
      }

      ctx.rect(
        col * CELL_SIZE,
        row * CELL_SIZE,
        CELL_SIZE,
        CELL_SIZE
      );
    }
  }

  ctx.fill();
};

requestAnimationFrame(renderLoop);
