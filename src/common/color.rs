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

pub const BRIGHTNESS_RANGE: usize = 5;
// パレット上のある色の輝度による変色の集合を表現するためのクラス
#[derive(Clone, Copy)]
pub struct ColorSet {
    colors: [ODColor; BRIGHTNESS_RANGE * 2 + 1],
    active_bright_range: usize,
    active_dark_range: usize,
}

impl ColorSet {
    pub const fn new(color: ODColor) -> Self {
        let colors: [ODColor; BRIGHTNESS_RANGE * 2 + 1] = [color; BRIGHTNESS_RANGE * 2 + 1];
        ColorSet {
            colors,
            active_bright_range: 0,
            active_dark_range: 0,
        }
    }

    pub fn get_color(&self, brightness: i32) -> Result<ODColor, String> {
        if i32::abs(brightness) > BRIGHTNESS_RANGE as i32
            || brightness > self.active_bright_range as i32
            || brightness < -(self.active_dark_range as i32)
        {
            return Err(format!("invalid brightness {brightness}"));
        }

        Ok(self.colors[(BRIGHTNESS_RANGE as i32 + brightness) as usize])
    }

    pub fn set_color(&mut self, brightness: i32, color: ODColor) -> Result<(), String> {
        if i32::abs(brightness) > BRIGHTNESS_RANGE as i32
            || brightness > self.active_bright_range as i32
            || brightness < -(self.active_dark_range as i32)
        {
            return Err(format!("invalid brightness {brightness}"));
        }

        self.colors[(BRIGHTNESS_RANGE as i32 + brightness) as usize] = color;
        Ok(())
    }

    pub fn add_brightness_range(&mut self) -> Result<(), String> {
        if self.active_bright_range >= BRIGHTNESS_RANGE {
            return Err(format!(
                "cannot add brightness range. current: {}",
                self.active_bright_range
            ));
        }

        self.active_bright_range += 1;
        Ok(())
    }

    pub fn dec_brightness_range(&mut self) -> Result<(), String> {
        if self.active_bright_range <= 0 {
            return Err(format!(
                "cannot decrease brightness range. current: {}",
                self.active_bright_range
            ));
        }

        self.active_bright_range -= 1;
        Ok(())
    }

    pub fn add_darkness_range(&mut self) -> Result<(), String> {
        if self.active_dark_range >= BRIGHTNESS_RANGE {
            return Err(format!(
                "cannot add darkness range. current: {}",
                self.active_dark_range
            ));
        }

        self.active_dark_range += 1;
        Ok(())
    }

    pub fn dec_darkness_range(&mut self) -> Result<(), String> {
        if self.active_dark_range <= 0 {
            return Err(format!(
                "cannot decrease darkness range. current: {}",
                self.active_dark_range
            ));
        }

        self.active_dark_range -= 1;
        Ok(())
    }

    pub fn get_active_brightness_range(&self) -> usize {
        self.active_bright_range
    }

    pub fn get_active_darkness_range(&self) -> usize {
        self.active_dark_range
    }
}

include!("tests/color_test.rs");
