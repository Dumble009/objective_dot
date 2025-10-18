use crate::{
    common::{drawing::Drawing, palette::PaletteColorIndex},
    ui_components::draw_modes::draw_mode::DrawMode,
};

use crate::actions::action::Action;

#[derive(Clone)]
pub struct Line {
    is_drawing: bool,
    drawing_start_pos: Option<(usize, usize)>,
}

impl Line {
    pub fn new() -> Self {
        Line {
            is_drawing: false,
            drawing_start_pos: None,
        }
    }

    fn bresenham(&self, mouse_pos: &(usize, usize), out_points: &mut Vec<(usize, usize)>) {
        let (mut x0, mut y0) = self.drawing_start_pos.unwrap_or((0, 0));
        let (mut x1, mut y1) = *mouse_pos;

        let xdiff = (x1 as i64 - x0 as i64).abs();
        let ydiff = (y1 as i64 - y0 as i64).abs();

        if xdiff < ydiff {
            // 傾斜が45度以下の直線にしか対応していないので、y軸の差分の方が大きい場合は
            // xとyを入れ替えることで、傾斜を45度以下にする
            std::mem::swap(&mut x0, &mut y0);
            std::mem::swap(&mut x1, &mut y1);
        }

        if x0 > x1 {
            // 常に x0 <= x1 になるようにする
            std::mem::swap(&mut x0, &mut x1);
            std::mem::swap(&mut y0, &mut y1);
        }

        let mut x = x0;
        let mut y = y0;

        let mut e = -(x1 as i64 - x0 as i64);

        out_points.push((x, y));
        while x < x1 {
            x += 1;

            e += 2 * (y1 as i64 - y0 as i64).abs();
            if e >= 0 {
                if y0 <= y1 {
                    y += 1;
                } else {
                    y -= 1;
                }
                e -= 2 * (x1 as i64 - x0 as i64);
            }
            out_points.push((x, y));
        }

        if xdiff < ydiff {
            // 入れ替えた場合は、再度xとyを入れ替える
            for point in out_points.iter_mut() {
                std::mem::swap(&mut point.0, &mut point.1);
            }
        }
    }

    fn horizontal_line(&self, mouse_pos: &(usize, usize), out_points: &mut Vec<(usize, usize)>) {
        let (mut x0, y0) = self.drawing_start_pos.unwrap_or((0, 0));
        let mut x1 = mouse_pos.0;

        if x0 > x1 {
            std::mem::swap(&mut x0, &mut x1);
        }

        for x in x0..=x1 {
            out_points.push((x, y0));
        }
    }

    fn vertical_line(&self, mouse_pos: &(usize, usize), out_points: &mut Vec<(usize, usize)>) {
        let (x0, mut y0) = self.drawing_start_pos.unwrap_or((0, 0));
        let mut y1 = mouse_pos.1;

        if y0 > y1 {
            std::mem::swap(&mut y0, &mut y1);
        }

        for y in y0..=y1 {
            out_points.push((x0, y));
        }
    }

    fn calc_line(&self, mouse_pos: &(usize, usize), out_points: &mut Vec<(usize, usize)>) {
        let (sx, sy) = self.drawing_start_pos.unwrap_or((0, 0));

        if sx == mouse_pos.0 {
            // 垂直線
            self.vertical_line(mouse_pos, out_points);
            return;
        }

        if sy == mouse_pos.1 {
            // 水平線
            self.horizontal_line(mouse_pos, out_points);
            return;
        }

        self.bresenham(mouse_pos, out_points);
    }
}

impl DrawMode for Line {
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
        self.calc_line(mouse_pos, &mut out_points);

        let current_selected_color_idx =
            drawing.get_palette().borrow().get_current_selected_idx()?;
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
        self.calc_line(mouse_pos, &mut out_points);

        let current_selected_color_idx =
            drawing.get_palette().borrow().get_current_selected_idx()?;
        for (x, y) in out_points {
            println!("point: ({x}, {y})");
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
    ) -> Result<Option<Box<dyn Action>>, String> {
        if !self.is_drawing {
            return Ok(None);
        }

        let mut out_points = Vec::new();
        self.calc_line(mouse_pos, &mut out_points);

        let current_selected_color_idx =
            drawing.get_palette().borrow().get_current_selected_idx()?;
        for (x, y) in out_points {
            if x < canvas_size.0 && y < canvas_size.1 {
                canvas[y][x] = current_selected_color_idx;
                drawing
                    .get_grid()
                    .borrow_mut()
                    .set_color(x, y, current_selected_color_idx)?;
            }
        }

        Ok(None)
    }

    fn get_button_label(&self) -> &str {
        "Line"
    }
}

include!("tests/line_test.rs");
