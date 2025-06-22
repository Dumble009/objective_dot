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

include!("tests/color_test.rs");
