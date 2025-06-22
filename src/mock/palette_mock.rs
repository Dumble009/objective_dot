use crate::common::color::ODColor;
use crate::common::palette::{Palette, PaletteColorIndex};

pub struct PaletteMock {
    colors: Vec<ODColor>,
    current_selected_idx: PaletteColorIndex,
}

impl PaletteMock {
    #[allow(dead_code)]
    pub fn new() -> Self {
        PaletteMock {
            colors: vec![ODColor::new(0, 0, 0)],
            current_selected_idx: 0,
        }
    }
}

impl Palette for PaletteMock {
    fn add_color(&mut self, color: ODColor) -> Result<(), String> {
        self.colors.push(color);
        Ok(())
    }

    fn get_color(&self, idx: PaletteColorIndex) -> Result<ODColor, String> {
        Ok(self.colors[idx])
    }

    fn get_color_count(&self) -> usize {
        self.colors.len()
    }

    fn get_current_selected_idx(&self) -> Result<PaletteColorIndex, String> {
        Ok(self.current_selected_idx)
    }

    fn select_color(&mut self, idx: PaletteColorIndex) -> Result<(), String> {
        self.current_selected_idx = idx;
        Ok(())
    }

    fn change_color(&mut self, idx: PaletteColorIndex, new_color: ODColor) -> Result<(), String> {
        self.colors[idx] = new_color;
        Ok(())
    }

    fn reset(&mut self) {
        self.colors.clear();
        self.colors.push(ODColor::new(0, 0, 0));
        self.current_selected_idx = 0;
    }
}
