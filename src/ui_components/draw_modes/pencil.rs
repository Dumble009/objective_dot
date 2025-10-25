use crate::{
    actions::draw_action::DrawAction,
    common::{drawing::Drawing, palette::PaletteColorIndex},
    ui_components::draw_modes::draw_mode::DrawMode,
};

use crate::actions::action::Action;

#[derive(Clone)]
pub struct Pencil {
    is_drawing: bool,
    drawn_cells: Vec<(usize, usize)>,
}

impl Pencil {
    pub fn new() -> Self {
        Pencil {
            is_drawing: false,
            drawn_cells: vec![],
        }
    }

    fn add_drawn_cell(
        &mut self,
        canvas_size: &(usize, usize),
        mouse_pos: &(usize, usize),
    ) -> Result<(), String> {
        if mouse_pos.0 >= canvas_size.0 || mouse_pos.1 >= canvas_size.1 {
            return Ok(());
        }
        self.drawn_cells.push(*mouse_pos);
        Ok(())
    }

    fn apply_current_drawn_cells_to_preview_canvas(
        &self,
        drawing: &mut dyn Drawing,
        preview_canvas: &mut [Vec<PaletteColorIndex>],
    ) -> Result<(), String> {
        let current_selected_color_idx =
            drawing.get_palette().borrow().get_current_selected_idx()?;

        for cell in self.drawn_cells.iter() {
            preview_canvas[cell.1][cell.0] = current_selected_color_idx;
        }

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
        self.drawn_cells.clear();

        self.add_drawn_cell(canvas_size, mouse_pos)?;
        self.apply_current_drawn_cells_to_preview_canvas(drawing, preview_canvas)?;

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

        self.add_drawn_cell(canvas_size, mouse_pos)?;
        self.apply_current_drawn_cells_to_preview_canvas(drawing, preview_canvas)?;

        Ok(())
    }

    fn on_mouse_up(
        &mut self,
        preview_canvas: &mut [Vec<PaletteColorIndex>],
        _canvas_size: &(usize, usize),
        drawing: &mut dyn Drawing,
        _mouse_pos: &(usize, usize),
    ) -> Result<Option<Box<dyn Action>>, String> {
        self.is_drawing = false;

        let current_selected_color_idx =
            drawing.get_palette().borrow().get_current_selected_idx()?;
        let action = Box::new(DrawAction::new(
            drawing.get_grid(),
            self.drawn_cells.clone(),
            current_selected_color_idx,
        ));

        self.apply_current_drawn_cells_to_preview_canvas(drawing, preview_canvas)?;

        Ok(Some(action))
    }

    fn get_button_label(&self) -> &str {
        "Pencil"
    }
}

include!("tests/pencil_test.rs");
