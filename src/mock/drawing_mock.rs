use std::cell::RefCell;
use std::rc::Rc;

use crate::common::drawing::Drawing;

use super::grid_mock::GridMock;
use super::palette_mock::PaletteMock;

use crate::common::canvas_grid::Grid;
use crate::common::palette::Palette;

pub struct DrawingMock {
    pub layers: Vec<Rc<RefCell<GridMock>>>,
    pub palette: Rc<RefCell<PaletteMock>>,
}

impl DrawingMock {
    // 使われている関数だが、コンパイラが正しく認識できていない
    #[allow(dead_code)]
    pub fn new() -> Self {
        DrawingMock {
            layers: vec![Rc::new(RefCell::new(GridMock::new()))],
            palette: Rc::new(RefCell::new(PaletteMock::new())),
        }
    }
}

impl Drawing for DrawingMock {
    fn get_grid(&self) -> Result<Box<dyn Grid>, String> {
        let mut grid = Box::new(GridMock::new());
        grid.set_grid_width(self.get_grid_width())?;
        grid.set_grid_height(self.get_grid_height())?;

        for y in 0..self.get_grid_height() {
            for x in 0..self.get_grid_width() {
                let color = self.layers[0].borrow().get_color(x, y)?;
                grid.set_color(x, y, color)?;
            }
        }

        Ok(grid)
    }

    fn get_grid_layer(&self, layer_index: usize) -> Option<Rc<RefCell<dyn Grid>>> {
        if layer_index < self.layers.len() {
            Some(self.layers[layer_index].clone())
        } else {
            None
        }
    }

    fn add_grid_layer(&mut self) {
        let new_layer = Rc::new(RefCell::new(GridMock::new()));
        new_layer
            .borrow_mut()
            .set_grid_width(self.get_grid_width())
            .unwrap();
        new_layer
            .borrow_mut()
            .set_grid_height(self.get_grid_height())
            .unwrap();
        self.layers.push(new_layer);
    }

    fn get_palette(&self) -> Rc<RefCell<dyn Palette>> {
        self.palette.clone()
    }

    fn get_grid_width(&self) -> usize {
        self.layers[0].borrow().get_grid_width()
    }

    fn get_grid_height(&self) -> usize {
        self.layers[0].borrow().get_grid_height()
    }

    fn set_grid_width(&mut self, w: usize) -> Result<(), String> {
        self.layers[0].borrow_mut().set_grid_width(w)
    }

    fn set_grid_height(&mut self, h: usize) -> Result<(), String> {
        self.layers[0].borrow_mut().set_grid_height(h)
    }

    fn get_layer_count(&self) -> usize {
        self.layers.len()
    }

    fn get_active_layer(&self) -> Rc<RefCell<dyn Grid>> {
        self.layers[0].clone()
    }

    fn set_active_layer_idx(&mut self, _layer_index: usize) -> Result<(), String> {
        Ok(())
    }

    fn move_layer_up(&mut self, _layer_index: usize) -> Result<(), String> {
        Ok(())
    }

    fn move_layer_down(&mut self, _layer_index: usize) -> Result<(), String> {
        Ok(())
    }
}
