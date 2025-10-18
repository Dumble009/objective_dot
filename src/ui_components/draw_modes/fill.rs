use crate::{
    common::{drawing::Drawing, palette::PaletteColorIndex},
    ui_components::draw_modes::draw_mode::DrawMode,
};

use crate::actions::action::Action;
use std::collections::VecDeque;

#[derive(Clone)]
pub struct Fill {}

impl Fill {
    pub fn new() -> Self {
        Fill {}
    }
}

impl DrawMode for Fill {
    fn on_mouse_down(
        &mut self,
        _canvas: &mut [Vec<PaletteColorIndex>],
        _canvas_size: &(usize, usize),
        _drawing: &mut dyn Drawing,
        _mouse_pos: &(usize, usize),
    ) -> Result<(), String> {
        Ok(())
    }

    fn on_mouse_drag(
        &mut self,
        _canvas: &mut [Vec<PaletteColorIndex>],
        _canvas_size: &(usize, usize),
        _drawing: &mut dyn Drawing,
        _mouse_pos: &(usize, usize),
    ) -> Result<(), String> {
        Ok(())
    }

    fn on_mouse_up(
        &mut self,
        canvas: &mut [Vec<PaletteColorIndex>],
        canvas_size: &(usize, usize),
        drawing: &mut dyn Drawing,
        mouse_pos: &(usize, usize),
    ) -> Result<Option<Box<dyn Action>>, String> {
        if mouse_pos.0 >= canvas_size.0 || mouse_pos.1 >= canvas_size.1 {
            return Ok(None);
        }

        let target_color = canvas[mouse_pos.1][mouse_pos.0]; // 塗りつぶし対象の色
        let fill_color = drawing.get_palette().borrow().get_current_selected_idx()?; // 塗りつぶし後の色

        if target_color == fill_color {
            // 塗りつぶし前後の色が同じ場合は何もする必要なし
            return Ok(None);
        }

        let grid = drawing.get_grid();

        let mut q = VecDeque::from([*mouse_pos]);
        canvas[mouse_pos.1][mouse_pos.0] = fill_color;
        grid.borrow_mut()
            .set_color(mouse_pos.0, mouse_pos.1, fill_color)?;

        while !q.is_empty() {
            let current = q.pop_front().unwrap(); // !is_empty なので絶対成功する

            let mut next_points = vec![];

            // 今見ている点の上下左右の点を次の候補としてピックアップ
            if current.1 > 0 {
                next_points.push((current.0, current.1 - 1));
            }
            if current.1 < canvas_size.1 - 1 {
                next_points.push((current.0, current.1 + 1));
            }
            if current.0 > 0 {
                next_points.push((current.0 - 1, current.1));
            }
            if current.0 < canvas_size.1 - 1 {
                next_points.push((current.0 + 1, current.1));
            }

            for next_point in &next_points {
                if canvas[next_point.1][next_point.0] != target_color {
                    continue;
                }

                canvas[next_point.1][next_point.0] = fill_color;
                grid.borrow_mut()
                    .set_color(next_point.0, next_point.1, fill_color)?;
                q.push_back(*next_point);
            }
        }

        Ok(None)
    }

    fn get_button_label(&self) -> &str {
        "Fill"
    }
}

include!("tests/fill_test.rs");
