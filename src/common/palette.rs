use crate::common::color::ODColor;

pub type PaletteColorIndex = usize;

pub trait Palette {
    fn add_color(&mut self, color: ODColor) -> Result<(), String>;
    fn get_color(&self, idx: PaletteColorIndex) -> Result<ODColor, String>;
    fn get_color_count(&self) -> usize;
    fn get_current_selected_idx(&self) -> Result<PaletteColorIndex, String>;
    fn select_color(&mut self, idx: PaletteColorIndex) -> Result<(), String>;
    fn change_color(&mut self, idx: PaletteColorIndex, new_color: ODColor) -> Result<(), String>;
    fn reset(&mut self);
    fn override_by_colorset(&mut self, colorset: &[ODColor]) -> Result<(), String>;
    fn remove_last_color(&mut self) -> Result<(), String>;
}

#[derive(Clone)]
pub struct ObjectPalette {
    colors: Vec<ODColor>,
    current_selected_idx: PaletteColorIndex,
}

const INITIAL_COLOR0: ODColor = ODColor::new(0, 0, 0);

impl ObjectPalette {
    pub fn new() -> Self {
        ObjectPalette {
            colors: vec![INITIAL_COLOR0],
            current_selected_idx: 0,
        }
    }
}

impl Palette for ObjectPalette {
    fn add_color(&mut self, color: ODColor) -> Result<(), String> {
        self.colors.push(color);
        Ok(())
    }

    fn get_color(&self, idx: PaletteColorIndex) -> Result<ODColor, String> {
        if idx >= self.colors.len() {
            return Err(format!("idx is invalid! idx:{idx}"));
        }

        Ok(self.colors[idx])
    }

    fn get_color_count(&self) -> usize {
        self.colors.len()
    }

    fn get_current_selected_idx(&self) -> Result<PaletteColorIndex, String> {
        if self.current_selected_idx >= self.get_color_count() {
            let current_selected_idx = self.current_selected_idx;
            return Err(format!(
                "cannot get selected color Index. idx: {current_selected_idx} is out of range."
            ));
        }

        Ok(self.current_selected_idx)
    }

    fn select_color(&mut self, idx: PaletteColorIndex) -> Result<(), String> {
        if idx >= self.get_color_count() {
            return Err(format!("cannot select color. idx: {idx} is out of range."));
        }

        self.current_selected_idx = idx;
        println!("selected color {idx}");
        Ok(())
    }

    fn change_color(&mut self, idx: PaletteColorIndex, new_color: ODColor) -> Result<(), String> {
        if idx >= self.get_color_count() {
            return Err(format!("cannnot chang color. idx: {idx} is out of range."));
        }

        self.colors[idx] = new_color;
        Ok(())
    }

    fn reset(&mut self) {
        self.colors.clear();
        self.colors.push(INITIAL_COLOR0);
        self.current_selected_idx = 0;
    }

    fn override_by_colorset(&mut self, colorset: &[ODColor]) -> Result<(), String> {
        if colorset.is_empty() {
            return Err(String::from("Color set is empty."));
        }

        self.colors.clear();
        for color in colorset {
            self.colors.push(*color);
        }

        Ok(())
    }

    fn remove_last_color(&mut self) -> Result<(), String> {
        if self.colors.len() <= 1 {
            return Err(String::from(
                "Called remove when there is only one color remaining.",
            ));
        }

        self.colors.pop();

        Ok(())
    }
}

include!("tests/palette_test.rs");
