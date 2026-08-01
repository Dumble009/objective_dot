use std::cell::RefCell;
use std::rc::Rc;

use crate::common::drawing::Drawing;

use super::grid_mock::GridMock;
use super::palette_mock::PaletteMock;

use crate::common::canvas_grid::Grid;
use crate::common::palette::Palette;

pub struct DrawingMock {
    pub grid: Rc<RefCell<GridMock>>,
    pub palette: Rc<RefCell<PaletteMock>>,
}

impl DrawingMock {
    // 使われている関数だが、コンパイラが正しく認識できていない
    #[allow(dead_code)]
    pub fn new() -> Self {
        DrawingMock {
            grid: Rc::new(RefCell::new(GridMock::new())),
            palette: Rc::new(RefCell::new(PaletteMock::new())),
        }
    }
}

impl Drawing for DrawingMock {
    fn get_grid(&self) -> Result<Rc<RefCell<dyn Grid>>, String> {
        let mut grid = Rc::new(RefCell::new(GridMock::new()));
        grid.borrow_mut().set_grid_width(self.get_grid_width())?;
        grid.borrow_mut().set_grid_height(self.get_grid_height())?;

        for y in 0..self.get_grid_height() {
            for x in 0..self.get_grid_width() {
                let color = self.grid.borrow().get_color(x, y)?;
                grid.borrow_mut().set_color(x, y, color)?;
            }
        }

        Ok(grid)
    }

    fn get_grid_layer(&self, layer_index: usize) -> Option<Rc<RefCell<dyn Grid>>> {
        if layer_index == 0 {
            Some(self.grid.clone())
        } else {
            None
        }
    }

    fn add_grid_layer(&mut self) {}

    fn get_palette(&self) -> Rc<RefCell<dyn Palette>> {
        self.palette.clone()
    }

    fn get_grid_width(&self) -> usize {
        self.grid.borrow().get_grid_width()
    }

    fn get_grid_height(&self) -> usize {
        self.grid.borrow().get_grid_height()
    }

    fn set_grid_width(&mut self, w: usize) -> Result<(), String> {
        self.grid.borrow_mut().set_grid_width(w)
    }

    fn set_grid_height(&mut self, h: usize) -> Result<(), String> {
        self.grid.borrow_mut().set_grid_height(h)
    }

    fn get_layer_count(&self) -> usize {
        1
    }

    fn get_active_layer(&self) -> Rc<RefCell<dyn Grid>> {
        self.grid.clone()
    }
}
