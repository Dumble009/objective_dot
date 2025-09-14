use std::cell::RefCell;
use std::rc::Rc;

use crate::action::action::Action;
use crate::common::color::ODColor;
use crate::common::palette::{Palette, PaletteColorIndex};

pub struct PaletteColorChangeAction {
    palette: Rc<RefCell<dyn Palette>>,
    idx: PaletteColorIndex,
    before_color: ODColor,
    after_color: ODColor,
}

impl PaletteColorChangeAction {
    pub fn new(
        p: Rc<RefCell<dyn Palette>>,
        i: PaletteColorIndex,
        bc: ODColor,
        ac: ODColor,
    ) -> Self {
        PaletteColorChangeAction {
            palette: p.clone(),
            idx: i,
            before_color: bc,
            after_color: ac,
        }
    }
}

impl Action for PaletteColorChangeAction {
    fn run(&mut self) -> Result<(), String> {
        self.palette
            .borrow_mut()
            .change_color(self.idx, self.after_color)
    }

    fn undo(&mut self) -> Result<(), String> {
        self.palette
            .borrow_mut()
            .change_color(self.idx, self.before_color)
    }
}

include!("tests/palette_color_change_action_test.rs");
