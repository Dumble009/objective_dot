use crate::common::drawing::Drawing;
use crate::common::palette::PaletteColorIndex;

pub trait DrawMode: DrawModeClone {
    fn on_mouse_down(
        &mut self,
        canvas: &mut [Vec<PaletteColorIndex>],
        canvas_size: &(usize, usize),
        drawing: &mut dyn Drawing,
        mouse_pos: &(usize, usize),
    ) -> Result<(), String>;
    fn on_mouse_drag(
        &mut self,
        canvas: &mut [Vec<PaletteColorIndex>],
        canvas_size: &(usize, usize),
        drawing: &mut dyn Drawing,
        mouse_pos: &(usize, usize),
    ) -> Result<(), String>;
    fn on_mouse_up(
        &mut self,
        canvas: &mut [Vec<PaletteColorIndex>],
        canvas_size: &(usize, usize),
        drawing: &mut dyn Drawing,
        mouse_pos: &(usize, usize),
    ) -> Result<(), String>;

    fn get_button_label(&self) -> &str;
}

pub trait DrawModeClone {
    fn clone_box(&self) -> Box<dyn DrawMode>;
}

impl<T> DrawModeClone for T
where
    T: 'static + DrawMode + Clone,
{
    fn clone_box(&self) -> Box<dyn DrawMode> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn DrawMode> {
    fn clone(&self) -> Box<dyn DrawMode> {
        self.clone_box()
    }
}
