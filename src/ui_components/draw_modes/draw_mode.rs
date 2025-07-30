use crate::common::drawing::Drawing;
use crate::common::palette::PaletteColorIndex;

pub trait DrawMode {
    fn on_mouse_down(
        &mut self,
        canvas: &mut Vec<Vec<PaletteColorIndex>>,
        canvas_size: &(usize, usize),
        drawing: &mut dyn Drawing,
        mouse_pos: &(usize, usize),
    ) -> Result<(), String>;
    fn on_mouse_drag(
        &mut self,
        canvas: &mut Vec<Vec<PaletteColorIndex>>,
        canvas_size: &(usize, usize),
        drawing: &mut dyn Drawing,
        mouse_pos: &(usize, usize),
    ) -> Result<(), String>;
    fn on_mouse_up(
        &mut self,
        canvas: &mut Vec<Vec<PaletteColorIndex>>,
        canvas_size: &(usize, usize),
        drawing: &mut dyn Drawing,
        mouse_pos: &(usize, usize),
    ) -> Result<(), String>;
}
