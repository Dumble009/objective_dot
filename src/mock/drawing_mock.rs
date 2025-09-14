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
    fn get_grid(&self) -> Rc<RefCell<dyn Grid>> {
        self.grid.clone()
    }

    fn get_palette(&self) -> Rc<RefCell<dyn Palette>> {
        self.palette.clone()
    }
}
