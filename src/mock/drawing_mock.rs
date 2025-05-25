use crate::common::drawing::Drawing;

use super::grid_mock::GridMock;
use super::palette_mock::PaletteMock;

use crate::common::canvas_grid::Grid;
use crate::common::palette::Palette;

pub struct DrawingMock {
    grid: GridMock,
    palette: PaletteMock,
}

impl DrawingMock {
    // 使われている関数だが、コンパイラが正しく認識できていない
    #[allow(dead_code)]
    pub fn new() -> Self {
        DrawingMock {
            grid: GridMock::new(),
            palette: PaletteMock::new(),
        }
    }
}

impl Drawing for DrawingMock {
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
