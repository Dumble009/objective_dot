use crate::common::palette::PaletteColorIndex;

#[derive(Default)]
pub struct CanvasGrid {
    grid: Vec<Vec<PaletteColorIndex>>,
}

const INITIAL_GRID_WIDTH: usize = 10;
const INITIAL_GRID_HEIGHT: usize = 10;

impl CanvasGrid {
    pub fn new() -> Self {
        CanvasGrid {
            grid: vec![vec![0; INITIAL_GRID_WIDTH]; INITIAL_GRID_HEIGHT],
        }
    }

    pub fn set_color(&mut self, x: usize, y: usize, val: PaletteColorIndex) -> Result<(), String> {
        self.coordinate_validation(x, y)?;

        self.grid[y][x] = val;

        Ok(())
    }

    pub fn get_color(&self, x: usize, y: usize) -> Result<PaletteColorIndex, String> {
        self.coordinate_validation(x, y)?;
        Ok(self.grid[y][x])
    }

    pub fn get_grid_width(&self) -> usize {
        INITIAL_GRID_WIDTH
    }

    pub fn get_grid_height(&self) -> usize {
        INITIAL_GRID_HEIGHT
    }

    fn coordinate_validation(&self, x: usize, y: usize) -> Result<(), String> {
        if y < self.grid.len() && x < self.grid[y].len() {
            Ok(())
        } else {
            Err(format!("args are out of range! x={x}, y={y}"))
        }
    }
}

include!("tests/canvas_grid_test.rs");
