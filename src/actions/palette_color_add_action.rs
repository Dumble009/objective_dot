use std::cell::RefCell;
use std::rc::Rc;

use crate::actions::action::Action;
use crate::common::color::ODColor;
use crate::common::palette::Palette;

pub struct PaletteColorAddAction {
    palette: Rc<RefCell<dyn Palette>>,
    color: ODColor,
}

impl PaletteColorAddAction {
    pub fn new(palette: Rc<RefCell<dyn Palette>>, color: ODColor) -> Self {
        PaletteColorAddAction {
            palette: palette.clone(),
            color,
        }
    }
}

impl Action for PaletteColorAddAction {
    fn run(&mut self) -> Result<(), String> {
        self.palette.borrow_mut().add_color(self.color)?;

        Ok(())
    }

    fn undo(&mut self) -> Result<(), String> {
        self.palette.borrow_mut().remove_last_color()?;

        Ok(())
    }
}

include!("tests/palette_color_add_action_test.rs");
