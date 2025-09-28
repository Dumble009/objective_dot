use std::cell::RefCell;
use std::rc::Rc;

use crate::actions::action::Action;
use crate::common::canvas_grid::Grid;

pub struct GridSizeChangeAction {
    grid: Rc<RefCell<dyn Grid>>,
    before_size: (usize, usize),
    after_size: (usize, usize),
}

impl GridSizeChangeAction {
    pub fn new(canvas: Rc<RefCell<dyn Grid>>, after_size: (usize, usize)) -> Self {
        GridSizeChangeAction {
            grid: canvas.clone(),
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
        self.grid.borrow_mut().set_grid_width(self.after_size.0)?;
        self.grid.borrow_mut().set_grid_height(self.after_size.1)?;
        Ok(())
    }

    fn undo(&mut self) -> Result<(), String> {
        self.grid.borrow_mut().set_grid_width(self.before_size.0)?;
        self.grid.borrow_mut().set_grid_height(self.before_size.1)?;
        Ok(())
    }
}

include!("tests/grid_size_change_action_test.rs");
