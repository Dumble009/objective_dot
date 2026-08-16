use crate::common::color::{ColorSet, ODColor};
use crate::common::palette::{Palette, PaletteColorIndex};

pub struct PaletteMock {
    colors: Vec<ColorSet>,
    current_selected_idx: PaletteColorIndex,
}

impl PaletteMock {
    #[allow(dead_code)]
    pub fn new() -> Self {
        PaletteMock {
            colors: vec![ColorSet::new(ODColor::new(0, 0, 0))],
            current_selected_idx: (0, 0).into(),
        }
    }
}

impl Palette for PaletteMock {
    fn add_color(&mut self, color: ODColor) -> Result<(), String> {
        self.colors.push(ColorSet::new(color));
        Ok(())
    }

    fn get_color(&self, idx: PaletteColorIndex) -> Result<ODColor, String> {
        self.colors[idx.idx].get_color(idx.brightness)
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
        self.colors[idx.idx].set_color(idx.brightness, new_color)
    }

    fn reset(&mut self) {
        self.colors.clear();
        self.colors.push(ColorSet::new(ODColor::new(0, 0, 0)));
        self.current_selected_idx = (0, 0).into();
    }

    fn override_by_color_sample(&mut self, _colorset: &[ODColor]) -> Result<(), String> {
        Ok(())
    }

    fn remove_last_color(&mut self) -> Result<(), String> {
        self.colors.pop();
        Ok(())
    }
}
