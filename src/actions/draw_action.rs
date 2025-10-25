use std::cell::RefCell;
use std::rc::Rc;

use crate::actions::action::Action;
use crate::common::canvas_grid::Grid;
use crate::common::palette::PaletteColorIndex;

pub struct DrawAction {
    canvas: Rc<RefCell<dyn Grid>>,
    drawn_cells: Vec<(usize, usize, usize)>,
    after_color_index: PaletteColorIndex,
}

impl DrawAction {
    pub fn new(
        canvas: Rc<RefCell<dyn Grid>>,
        cells: Vec<(usize, usize)>,
        after_color_index: PaletteColorIndex,
    ) -> Self {
        let mut drawn_cells = vec![];
        for cell in cells {
            let x = cell.0;
            let y = cell.1;
            drawn_cells.push((x, y, canvas.borrow().get_color(x, y).unwrap()));
        }

        DrawAction {
            canvas: canvas.clone(),
            drawn_cells,
            after_color_index,
        }
    }
}

impl Action for DrawAction {
    fn run(&mut self) -> Result<(), String> {
        for drawn_cell in self.drawn_cells.iter() {
            self.canvas.borrow_mut().set_color(
                drawn_cell.0,
                drawn_cell.1,
                self.after_color_index,
            )?;
        }
        Ok(())
    }

    fn undo(&mut self) -> Result<(), String> {
        for drawn_cell in self.drawn_cells.iter() {
            self.canvas
                .borrow_mut()
                .set_color(drawn_cell.0, drawn_cell.1, drawn_cell.2)?;
        }
        Ok(())
    }
}

include!("tests/draw_action_test.rs");
