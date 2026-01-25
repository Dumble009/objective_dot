use crate::common::drawing::Drawing;

pub struct Bitmap {
    pub pixels: Vec<u8>, // RGBA format
    pub width: usize,
    pub height: usize,
}

impl Bitmap {
    pub fn from_drawing(drawing: &dyn Drawing) -> Result<Self, String> {
        let grid = drawing.get_grid();
        let palette = drawing.get_palette();

        let width = grid.borrow().get_grid_width();
        let height = grid.borrow().get_grid_height();

        let mut pixels = Vec::with_capacity(width * height * 4);

        for y in 0..height {
            for x in 0..width {
                let color_index = grid.borrow().get_color(x, y)?;
                let od_color = palette.borrow().get_color(color_index)?;
                let color32 = od_color.to_color32();

                pixels.push(color32.r());
                pixels.push(color32.g());
                pixels.push(color32.b());
                pixels.push(255); // Alpha channel
            }
        }

        Ok(Bitmap {
            pixels,
            width,
            height,
        })
    }
}

include!("tests/bitmap_test.rs");
