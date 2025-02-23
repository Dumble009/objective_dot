use egui::Color32;

#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub struct ODColor {
    r: u8,
    g: u8,
    b: u8,
}

impl ODColor {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        ODColor { r, g, b }
    }

    pub fn to_color32(self) -> Color32 {
        Color32::from_rgb(self.r, self.g, self.b)
    }
}

include!("tests/color_test.rs");
