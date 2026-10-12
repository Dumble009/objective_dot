use std::cell::RefCell;
use std::rc::Rc;

use crate::actions::action::Action;
use crate::common::palette::Palette;

pub struct PaletteBrightnessAddAction {
    palette: Rc<RefCell<dyn Palette>>,
    idx: usize,
}

impl PaletteBrightnessAddAction {
    pub fn new(palette: Rc<RefCell<dyn Palette>>, idx: usize) -> Self {
        PaletteBrightnessAddAction {
            palette: palette.clone(),
            idx,
        }
    }
}

impl Action for PaletteBrightnessAddAction {
    fn run(&mut self) -> Result<(), String> {
        self.palette.borrow_mut().add_brightness_range(self.idx)
    }

    fn undo(&mut self) -> Result<(), String> {
        self.palette.borrow_mut().dec_brightness_range(self.idx)
    }
}

include!("tests/palette_brightness_add_action_test.rs");
