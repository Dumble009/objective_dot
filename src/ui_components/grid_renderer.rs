use eframe::egui::*;

use crate::common::palette::{Palette, PaletteColorIndex};

pub trait GridRenderer {
    fn draw(
        &self,
        ui: &mut Ui,
        canvas: &[Vec<PaletteColorIndex>],
        palette: &dyn Palette,
        square_root_pos: Pos2,
        square_size: f32,
        offset: Vec2,
    ) -> Result<(), String>;
}

pub struct SimpleGridRenderer;

impl SimpleGridRenderer {
    pub fn new() -> Self {
        SimpleGridRenderer
    }
}

impl GridRenderer for SimpleGridRenderer {
    fn draw(
        &self,
        ui: &mut Ui,
        canvas: &[Vec<PaletteColorIndex>],
        palette: &dyn Palette,
        square_root_pos: Pos2,
        square_size: f32,
        offset: Vec2,
    ) -> Result<(), String> {
        for (y, row) in canvas.iter().enumerate() {
            for (x, color_idx) in row.iter().enumerate() {
                let square_pos = square_root_pos
                    + Vec2::new(x as f32 * square_size, y as f32 * square_size)
                    + offset;
                let square_rect = Rect::from_min_max(
                    square_pos,
                    square_pos + Vec2::new(square_size, square_size),
                );

                let color = palette.get_color(*color_idx)?;
                let fill_color = color.to_color32();

                let stroke_color = Color32::from_rgb(
                    255 - fill_color.r(),
                    255 - fill_color.g(),
                    255 - fill_color.b(),
                );
                let grid_stroke = Stroke::new(1.0, stroke_color);

                ui.painter()
                    .rect(square_rect, 0, fill_color, grid_stroke, StrokeKind::Middle);
            }
        }

        Ok(())
    }
}
