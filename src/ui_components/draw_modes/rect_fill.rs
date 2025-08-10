use crate::{
    common::{drawing::Drawing, palette::PaletteColorIndex},
    ui_components::draw_modes::draw_mode::DrawMode,
};

#[derive(Clone)]
pub struct RectFill {
    is_drawing: bool,
    drawing_start_pos: Option<(usize, usize)>,
}

impl RectFill {
    pub fn new() -> Self {
        RectFill {
            is_drawing: false,
            drawing_start_pos: None,
        }
    }

    fn calc_rect_points(&self, mouse_pos: &(usize, usize), out_points: &mut Vec<(usize, usize)>) {
        let (x0, y0) = self.drawing_start_pos.unwrap_or((0, 0));
        let (x1, y1) = *mouse_pos;
        let x_min = x0.min(x1);
        let x_max = x0.max(x1);
        let y_min = y0.min(y1);
        let y_max = y0.max(y1);

        for y in y_min..=y_max {
            for x in x_min..=x_max {
                out_points.push((x, y));
            }
        }
    }
}

impl DrawMode for RectFill {
    fn on_mouse_down(
        &mut self,
        canvas: &mut [Vec<PaletteColorIndex>],
        canvas_size: &(usize, usize),
        drawing: &mut dyn Drawing,
        mouse_pos: &(usize, usize),
    ) -> Result<(), String> {
        self.is_drawing = true;
        self.drawing_start_pos = Some(*mouse_pos);

        let mut out_points = Vec::new();
        self.calc_rect_points(mouse_pos, &mut out_points);

        let current_selected_color_idx = drawing.get_palette().get_current_selected_idx()?;
        for (x, y) in out_points {
            if x < canvas_size.0 && y < canvas_size.1 {
                canvas[y][x] = current_selected_color_idx;
            }
        }
        Ok(())
    }

    fn on_mouse_drag(
        &mut self,
        canvas: &mut [Vec<PaletteColorIndex>],
        canvas_size: &(usize, usize),
        drawing: &mut dyn Drawing,
        mouse_pos: &(usize, usize),
    ) -> Result<(), String> {
        if !self.is_drawing {
            return Ok(());
        }
        let mut out_points = Vec::new();
        self.calc_rect_points(mouse_pos, &mut out_points);

        let current_selected_color_idx = drawing.get_palette().get_current_selected_idx()?;
        for (x, y) in out_points {
            if x < canvas_size.0 && y < canvas_size.1 {
                canvas[y][x] = current_selected_color_idx;
            }
        }
        Ok(())
    }

    fn on_mouse_up(
        &mut self,
        canvas: &mut [Vec<PaletteColorIndex>],
        canvas_size: &(usize, usize),
        drawing: &mut dyn Drawing,
        mouse_pos: &(usize, usize),
    ) -> Result<(), String> {
        if !self.is_drawing {
            return Ok(());
        }
        let mut out_points = Vec::new();
        self.calc_rect_points(mouse_pos, &mut out_points);

        let current_selected_color_idx = drawing.get_palette().get_current_selected_idx()?;
        for (x, y) in out_points {
            if x < canvas_size.0 && y < canvas_size.1 {
                canvas[y][x] = current_selected_color_idx;
                drawing
                    .get_grid_mut()
                    .set_color(x, y, current_selected_color_idx)?;
            }
        }
        self.is_drawing = false;
        self.drawing_start_pos = None;
        Ok(())
    }

    fn get_button_label(&self) -> &str {
        "RectFill"
    }
}

include!("tests/rect_fill_test.rs");
