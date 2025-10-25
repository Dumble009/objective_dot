use crate::{
    common::{drawing::Drawing, palette::PaletteColorIndex},
    ui_components::draw_modes::draw_mode::DrawMode,
};

use crate::actions::action::Action;

#[derive(Clone)]
pub struct Pencil {
    is_drawing: bool,
}

impl Pencil {
    pub fn new() -> Self {
        Pencil { is_drawing: false }
    }

    fn put_color_on_cell(
        &self,
        preview_canvas: &mut [Vec<PaletteColorIndex>],
        canvas_size: &(usize, usize),
        drawing: &mut dyn Drawing,
        mouse_pos: &(usize, usize),
    ) -> Result<(), String> {
        if mouse_pos.0 >= canvas_size.0 || mouse_pos.1 >= canvas_size.1 {
            return Ok(());
        }

        let current_selected_color_idx =
            drawing.get_palette().borrow().get_current_selected_idx()?;
        preview_canvas[mouse_pos.1][mouse_pos.0] = current_selected_color_idx;
        drawing.get_grid().borrow_mut().set_color(
            mouse_pos.0,
            mouse_pos.1,
            current_selected_color_idx,
        )?;

        Ok(())
    }
}

impl DrawMode for Pencil {
    fn on_mouse_down(
        &mut self,
        preview_canvas: &mut [Vec<PaletteColorIndex>],
        canvas_size: &(usize, usize),
        drawing: &mut dyn Drawing,
        mouse_pos: &(usize, usize),
    ) -> Result<(), String> {
        self.is_drawing = true;

        if mouse_pos.0 >= canvas_size.0 || mouse_pos.1 >= canvas_size.1 {
            // 範囲外でマウスを押下されていたとしても、描画開始だけして許容する
            return Ok(());
        }

        self.put_color_on_cell(preview_canvas, canvas_size, drawing, mouse_pos)?;

        Ok(())
    }

    fn on_mouse_drag(
        &mut self,
        preview_canvas: &mut [Vec<PaletteColorIndex>],
        canvas_size: &(usize, usize),
        drawing: &mut dyn Drawing,
        mouse_pos: &(usize, usize),
    ) -> Result<(), String> {
        if !self.is_drawing {
            return Ok(());
        }

        if mouse_pos.0 >= canvas_size.0 || mouse_pos.1 >= canvas_size.1 {
            // 範囲外でマウスをドラッグされていたとしても、描画はしない
            return Ok(());
        }

        self.put_color_on_cell(preview_canvas, canvas_size, drawing, mouse_pos)?;

        Ok(())
    }

    fn on_mouse_up(
        &mut self,
        _preview_canvas: &mut [Vec<PaletteColorIndex>],
        _canvas_size: &(usize, usize),
        _drawing: &mut dyn Drawing,
        _mouse_pos: &(usize, usize),
    ) -> Result<Option<Box<dyn Action>>, String> {
        self.is_drawing = false;
        Ok(None)
    }

    fn get_button_label(&self) -> &str {
        "Pencil"
    }
}

include!("tests/pencil_test.rs");
