use egui::Color32;

#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub struct ODColor {
    r: u8,
    g: u8,
    b: u8,
}

impl ODColor {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        ODColor { r, g, b }
    }

    pub fn to_color32(self) -> Color32 {
        Color32::from_rgb(self.r, self.g, self.b)
    }

    pub fn from_color32(c32: Color32) -> Self {
        ODColor::new(c32.r(), c32.g(), c32.b())
    }
}

pub const BRIGHTNESS_RANGE: usize = 10;
pub struct ColorSet {
    colors: [ODColor; BRIGHTNESS_RANGE * 2 + 1],
}

impl ColorSet {
    pub const fn new(color: ODColor) -> Self {
        let colors: [ODColor; BRIGHTNESS_RANGE * 2 + 1] = [color; BRIGHTNESS_RANGE * 2 + 1];
        ColorSet { colors }
    }

    pub fn get_color(&self, brightness: i32) -> Result<ODColor, String> {
        if i32::abs(brightness) > BRIGHTNESS_RANGE as i32 {
            return Err(format!("invalid brightness {brightness}"));
        }

        Ok(self.colors[BRIGHTNESS_RANGE + brightness as usize])
    }

    pub fn set_color(&mut self, brightness: i32, color: ODColor) -> Result<(), String> {
        if i32::abs(brightness) > BRIGHTNESS_RANGE as i32 {
            return Err(format!("invalid brightness {brightness}"));
        }

        self.colors[BRIGHTNESS_RANGE + brightness as usize] = color;
        Ok(())
    }
}

include!("tests/color_test.rs");
