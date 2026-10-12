use std::cell::RefCell;
use std::rc::Rc;

use crate::actions::action::Action;
use crate::common::color::ODColor;
use crate::common::palette::{Palette, PaletteColorIndex};

pub struct PaletteSelectColorAction {
    palette: Rc<RefCell<dyn Palette>>,
    before_idx: PaletteColorIndex,
    after_idx: PaletteColorIndex,
}

impl PaletteSelectColorAction {
    pub fn new(
        p: Rc<RefCell<dyn Palette>>,
        before_idx: PaletteColorIndex,
        after_idx: PaletteColorIndex,
    ) -> Self {
        PaletteSelectColorAction {
            palette: p.clone(),
            before_idx,
            after_idx,
        }
    }
}

impl Action for PaletteSelectColorAction {
    fn run(&mut self) -> Result<(), String> {
        self.palette.borrow_mut().select_color(self.after_idx)
    }

    fn undo(&mut self) -> Result<(), String> {
        self.palette.borrow_mut().select_color(self.before_idx)
    }
}

include!("tests/palette_select_color_action_test.rs");
