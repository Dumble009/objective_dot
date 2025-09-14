use std::cell::RefCell;
use std::rc::Rc;

use super::canvas_grid::{CanvasGrid, Grid};
use super::palette::{ObjectPalette, Palette};

pub trait Drawing {
    fn get_grid(&self) -> Rc<RefCell<dyn Grid>>;
    fn get_palette(&self) -> Rc<RefCell<dyn Palette>>;
}

pub struct ObjectDrawing {
    grid: Rc<RefCell<CanvasGrid>>,
    palette: Rc<RefCell<ObjectPalette>>,
}

impl ObjectDrawing {
    pub fn new() -> Self {
        ObjectDrawing {
            palette: Rc::new(RefCell::new(ObjectPalette::new())),
            grid: Rc::new(RefCell::new(CanvasGrid::new())),
        }
    }
}

impl Drawing for ObjectDrawing {
    fn get_grid(&self) -> Rc<RefCell<dyn Grid>> {
        self.grid.clone()
    }

    fn get_palette(&self) -> Rc<RefCell<dyn Palette>> {
        self.palette.clone()
    }
}
