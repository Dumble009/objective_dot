use crate::common::color::ODColor;

pub type PaletteColorIndex = usize;

#[derive(Clone)]
pub struct Palette {
    colors: Vec<ODColor>,
    current_selected_idx: PaletteColorIndex,
}

impl Palette {
    pub fn new() -> Self {
        Palette {
            colors: vec![ODColor::new(0, 0, 0)],
            current_selected_idx: 0,
        }
    }

    pub fn add_color(&mut self, color: ODColor) -> Result<(), String> {
        self.colors.push(color);
        Ok(())
    }

    pub fn get_color(&self, idx: PaletteColorIndex) -> Result<ODColor, String> {
        if idx >= self.colors.len() {
            return Err(format!("idx is invalid! idx:{idx}"));
        }

        Ok(self.colors[idx])
    }

    pub fn get_color_count(&self) -> usize {
        self.colors.len()
    }

    pub fn get_current_active_idx(&self) -> Result<PaletteColorIndex, String> {
        if self.current_selected_idx >= self.get_color_count() {
            let current_selected_idx = self.current_selected_idx;
            return Err(format!(
                "cannot get active color Index. idx: {current_selected_idx} is out of range."
            ));
        }

        Ok(self.current_selected_idx)
    }

    pub fn select_color(&mut self, idx: PaletteColorIndex) -> Result<(), String> {
        if idx >= self.get_color_count() {
            return Err(format!("cannot select color. idx: {idx} is out of range."));
        }

        self.current_selected_idx = idx;
        Ok(())
    }
}

include!("tests/palette_test.rs");
