use crate::actions::action::Action;
use crate::common::drawing::Drawing;
use crate::common::palette::PaletteColorIndex;

use std::cell::RefCell;
use std::rc::Rc;

// preview_canvas は例えば四角を描く場合に、マウスドラッグ中に描画されるプレビューのためのキャンバス
// 実際には反映されていないが、今マウスを離すとこういう風になる、というのを分かるようにするためのもの
pub trait DrawMode: DrawModeClone {
    fn on_mouse_down(
        &mut self,
        preview_canvas: &mut [Vec<PaletteColorIndex>],
        canvas_size: &(usize, usize),
        drawing: Rc<RefCell<dyn Drawing>>,
        mouse_pos: &(usize, usize),
    ) -> Result<(), String>;
    fn on_mouse_drag(
        &mut self,
        preview_canvas: &mut [Vec<PaletteColorIndex>],
        canvas_size: &(usize, usize),
        drawing: Rc<RefCell<dyn Drawing>>,
        mouse_pos: &(usize, usize),
    ) -> Result<(), String>;
    fn on_mouse_up(
        &mut self,
        preview_canvas: &mut [Vec<PaletteColorIndex>],
        canvas_size: &(usize, usize),
        drawing: Rc<RefCell<dyn Drawing>>,
        mouse_pos: &(usize, usize),
    ) -> Result<Option<Box<dyn Action>>, String>;

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
