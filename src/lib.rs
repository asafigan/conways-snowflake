mod utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Universe {
    width: u32,
    height: u32,
    last_generation: Vec<Cell>,
    next_generation: Vec<Cell>,
}

#[wasm_bindgen]
impl Universe {
    pub fn new(width: u32, height: u32) -> Universe {
        utils::set_panic_hook();

        let cells: Vec<_> = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Universe {
            width,
            height,
            last_generation: cells.clone(),
            next_generation: cells,
        }
    }

    pub fn render_into(&self, buffer: &mut [u8]) {
        static BLACK: u32 = 0xFF_00_00_00;
        static WHITE: u32 = 0xFF_FF_FF_FF;
        let (_, pixels, _) = unsafe { buffer.align_to_mut::<u32>() };
        for (cell, pixel) in self.next_generation.iter().zip(pixels.iter_mut()) {
            match cell {
                Cell::Alive => *pixel = BLACK,
                Cell::Dead => *pixel = WHITE,
            }
        }
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;

        let north = if row == 0 { self.height - 1 } else { row - 1 };

        let south = if row == self.height - 1 { 0 } else { row + 1 };

        let west = if column == 0 {
            self.width - 1
        } else {
            column - 1
        };

        let east = if column == self.width - 1 {
            0
        } else {
            column + 1
        };

        let nw = self.get_index(north, west);
        count += self.last_generation[nw] as u8;

        let n = self.get_index(north, column);
        count += self.last_generation[n] as u8;

        let ne = self.get_index(north, east);
        count += self.last_generation[ne] as u8;

        let w = self.get_index(row, west);
        count += self.last_generation[w] as u8;

        let e = self.get_index(row, east);
        count += self.last_generation[e] as u8;

        let sw = self.get_index(south, west);
        count += self.last_generation[sw] as u8;

        let s = self.get_index(south, column);
        count += self.last_generation[s] as u8;

        let se = self.get_index(south, east);
        count += self.last_generation[se] as u8;

        count
    }

    pub fn tick(&mut self) {
        std::mem::swap(&mut self.last_generation, &mut self.next_generation);

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.last_generation[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    (_, 3) => Cell::Alive,
                    (Cell::Alive, 2) => Cell::Alive,
                    _ => Cell::Dead,
                };

                self.next_generation[idx] = next_cell;
            }
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn clear(&mut self) {
        self.next_generation = (0..self.width * self.height).map(|_i| Cell::Dead).collect();
    }
}

#[wasm_bindgen]
pub fn create_buffer(size: usize) -> Clamped<Vec<u8>> {
    Clamped(vec![0; size])
}

impl Universe {
    /// Set cells to be alive in a universe by passing the row and column
    /// of each cell as an array.
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.next_generation[idx] = Cell::Alive;
        }
    }
}
