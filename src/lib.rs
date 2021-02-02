mod utils;
extern crate fixedbitset;

use wasm_bindgen::prelude::*;
use fixedbitset::FixedBitSet;


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet,
    // new cells to toggle on each tick
    new_cells: FixedBitSet,
}

impl Universe {
    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }
    
    fn get_new_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    fn live_neighbours (&self, row: u32, col: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbour_row = (row + delta_row) % self.height;
                let neighbour_col = (col + delta_col) % self.width;
                let id = self.get_index(neighbour_row, neighbour_col);
                count += self.cells[id] as u8
            }
        }
        count
    }

    pub fn get_cells(&self) -> &fixedbitset::FixedBitSet {
        return &self.cells;
    }
}

use std::fmt;

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if !self.cells[cell as usize] { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

// public methods exported to javascript
#[wasm_bindgen]
impl Universe {
    pub fn toggle_cell (&mut self, row: u32, col: u32) {
        let id = self.get_index(row, col);
        self.new_cells.set(id, true);
    }

    pub fn set_size (&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;

        let size = (width * height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);

        for i in 0..size {
            cells.set(i, false);
        }

        self.cells = cells;
    }

    pub fn tick (&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let id = self.get_index(row, col);
                if self.new_cells[id] {
                    next.set(id, true);
                    self.new_cells.set(id, false);
                } else {
                    let cell = self.cells[id];
                    let live_neighbours = self.live_neighbours(row, col);
                    next.set(id, match (cell, live_neighbours) {
                        // Rule 1: Any live cell with fewer than two live neighbours
                        // dies, as if caused by underpopulation.
                        (true, x) if x < 2 => false,
                        // Rule 2: Any live cell with two or three live neighbours
                        // lives on to the next generation.
                        (true, 2) | (true, 3) => true,
                        // Rule 3: Any live cell with more than three live
                        // neighbours dies, as if by overpopulation.
                        (true, x) if x > 3 => false,
                        // Rule 4: Any dead cell with exactly three live neighbours
                        // becomes a live cell, as if by reproduction.
                        (false, 3) => true,
                        (otherwise, _) => otherwise
                    })
                }
            }
        }
        self.cells = next;
    }

    pub fn new(width: u32, height: u32) -> Universe {
        let size = (width * height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);
        let mut new_cells = FixedBitSet::with_capacity(size);

        for i in 0..size {
            cells.set(i, js_sys::Math::random() > 0.5);
            new_cells.set(i, false);
        }

        Universe {
            width,
            height,
            cells,
            new_cells
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn width (&self) -> u32 {
        self.width
    }

    pub fn height (&self) -> u32 {
        self.height
    }

    pub fn cells (&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }
}

