use std::cell::RefCell;
use std::rc::Rc;

use crate::actions::action::Action;
use crate::common::palette::Palette;

pub struct PaletteDarknessAddAction {
    palette: Rc<RefCell<dyn Palette>>,
    idx: usize,
}

impl PaletteDarknessAddAction {
    pub fn new(palette: Rc<RefCell<dyn Palette>>, idx: usize) -> Self {
        PaletteDarknessAddAction {
            palette: palette.clone(),
            idx,
        }
    }
}

impl Action for PaletteDarknessAddAction {
    fn run(&mut self) -> Result<(), String> {
        self.palette.borrow_mut().add_darkness_range(self.idx)
    }

    fn undo(&mut self) -> Result<(), String> {
        self.palette.borrow_mut().dec_darkness_range(self.idx)
    }
}

include!("tests/palette_darkness_add_action_test.rs");
