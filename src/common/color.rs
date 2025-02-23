pub struct ODColor {
    r: u8,
    g: u8,
    b: u8,
}

impl ODColor {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        ODColor { r, g, b }
    }
}
