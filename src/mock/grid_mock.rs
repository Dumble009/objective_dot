use crate::common::{canvas_grid::Grid, palette::PaletteColorIndex};
use std::collections::HashMap;

pub struct GridMock {
    colors: HashMap<(usize, usize), PaletteColorIndex>,
    width: usize,
    height: usize,
}

impl GridMock {
    // 使われている関数だが、コンパイラが正しく認識できていない
    #[allow(dead_code)]
    pub fn new() -> Self {
        GridMock {
            colors: HashMap::new(),
            width: 10,
            height: 10,
        }
    }
}

impl Grid for GridMock {
    fn get_color(&self, x: usize, y: usize) -> Result<PaletteColorIndex, String> {
        if self.colors.contains_key(&(x, y)) {
            Ok(self.colors[&(x, y)])
        } else {
            Ok(0)
        }
    }

    fn set_color(&mut self, x: usize, y: usize, val: PaletteColorIndex) -> Result<(), String> {
        self.colors.insert((x, y), val);
        Ok(())
    }

    fn get_grid_width(&self) -> usize {
        self.width
    }

    fn get_grid_height(&self) -> usize {
        self.height
    }

    fn set_grid_width(&mut self, new_w: usize) -> Result<(), String> {
        self.width = new_w;
        Ok(())
    }

    fn set_grid_height(&mut self, new_h: usize) -> Result<(), String> {
        self.height = new_h;
        Ok(())
    }
}
