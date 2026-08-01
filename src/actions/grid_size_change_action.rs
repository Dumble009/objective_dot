use std::cell::RefCell;
use std::rc::Rc;

use crate::actions::action::Action;
use crate::common::drawing::Drawing;

pub struct GridSizeChangeAction {
    drawing: Rc<RefCell<dyn Drawing>>,
    before_size: (usize, usize),
    after_size: (usize, usize),
}

impl GridSizeChangeAction {
    pub fn new(canvas: Rc<RefCell<dyn Drawing>>, after_size: (usize, usize)) -> Self {
        GridSizeChangeAction {
            drawing: canvas.clone(),
            before_size: (
                canvas.borrow().get_grid_width(),
                canvas.borrow().get_grid_height(),
            ),
            after_size,
        }
    }
}

impl Action for GridSizeChangeAction {
    fn run(&mut self) -> Result<(), String> {
        self.drawing
            .borrow_mut()
            .set_grid_width(self.after_size.0)?;
        self.drawing
            .borrow_mut()
            .set_grid_height(self.after_size.1)?;
        Ok(())
    }

    fn undo(&mut self) -> Result<(), String> {
        self.drawing
            .borrow_mut()
            .set_grid_width(self.before_size.0)?;
        self.drawing
            .borrow_mut()
            .set_grid_height(self.before_size.1)?;
        Ok(())
    }
}

include!("tests/grid_size_change_action_test.rs");
