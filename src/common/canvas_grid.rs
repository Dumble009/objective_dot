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
        if self.get_grid_height() == 0 {
            return 0;
        }
        self.grid[0].len()
    }

    pub fn get_grid_height(&self) -> usize {
        self.grid.len()
    }

    pub fn set_grid_width(&mut self, new_w: usize) -> Result<(), String> {
        if new_w == 0 {
            return Err(String::from("cannot set grid width 0."));
        }

        let old_w = self.get_grid_width();

        if new_w == old_w {
            return Ok(());
        }

        if new_w > old_w {
            let diff = new_w - old_w;
            for row in self.grid.iter_mut() {
                for _ in 0..diff {
                    row.push(0);
                }
            }
        } else {
            let diff = old_w - new_w;
            for row in self.grid.iter_mut() {
                for _ in 0..diff {
                    row.pop();
                }
            }
        }

        Ok(())
    }

    pub fn set_grid_height(&mut self, new_h: usize) -> Result<(), String> {
        if new_h == 0 {
            return Err(String::from("cannot set grid height 0."));
        }

        let old_h = self.get_grid_height();

        if new_h == old_h {
            return Ok(());
        }

        if new_h > old_h {
            let diff = new_h - old_h;
            let width = self.get_grid_width();
            for _ in 0..diff {
                self.grid.push(vec![0; width]);
            }
        } else {
            let diff = old_h - new_h;
            for _ in 0..diff {
                self.grid.pop();
            }
        }

        Ok(())
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
