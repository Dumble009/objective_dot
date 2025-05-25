use super::canvas_grid::{CanvasGrid, Grid};
use super::palette::{ObjectPalette, Palette};

pub trait Drawing {
    fn get_grid(&self) -> &dyn Grid;
    fn get_grid_mut(&mut self) -> &mut dyn Grid;

    fn get_palette(&self) -> &dyn Palette;
    fn get_palette_mut(&mut self) -> &mut dyn Palette;
}

pub struct ObjectDrawing {
    grid: CanvasGrid,
    palette: ObjectPalette,
}

impl ObjectDrawing {
    pub fn new() -> Self {
        ObjectDrawing {
            palette: ObjectPalette::new(),
            grid: CanvasGrid::new(),
        }
    }
}

impl Drawing for ObjectDrawing {
    fn get_grid(&self) -> &dyn Grid {
        &self.grid
    }

    fn get_grid_mut(&mut self) -> &mut dyn Grid {
        &mut self.grid
    }

    fn get_palette(&self) -> &dyn Palette {
        &self.palette
    }

    fn get_palette_mut(&mut self) -> &mut dyn Palette {
        &mut self.palette
    }
}
