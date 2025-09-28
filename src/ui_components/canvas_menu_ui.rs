use std::{cell::RefCell, rc::Rc};

use eframe::egui::*;

use crate::actions::action::Action;
use crate::actions::grid_size_change_action::GridSizeChangeAction;
use crate::common::canvas_grid::Grid;

use super::top_menu_bar_item::TopMenuBarItem;
use std::collections::VecDeque;

pub struct CanvasMenuUi {
    is_showing: bool,
}

impl CanvasMenuUi {
    pub fn new() -> Self {
        CanvasMenuUi { is_showing: false }
    }

    fn draw(
        &mut self,
        ui: &mut Ui,
        grid: Rc<RefCell<dyn Grid>>,
        action_q: &mut VecDeque<Box<dyn Action>>,
    ) {
        let mut layout = Layout::left_to_right(Align::Min);
        layout.main_wrap = true;

        let mut width = grid.borrow().get_grid_width();
        let mut height = grid.borrow().get_grid_height();
        let before_width = width;
        let before_height = height;

        ui.horizontal(|ui| {
            ui.label("Width "); // Height と文字数を揃えるためにスペースを入れている
            ui.add(DragValue::new(&mut width).update_while_editing(false));
        });

        ui.horizontal(|ui| {
            ui.label("Height");
            ui.add(DragValue::new(&mut height).update_while_editing(false));
        });

        // アクションキューを無駄に消費しないように、値の変更が確定した時だけキューに積む
        if before_width != width || before_height != height {
            let action = GridSizeChangeAction::new(grid.clone(), (width, height));
            action_q.push_front(Box::new(action));
        }

        if ui.button("split").clicked() {
            let res = grid.borrow_mut().split();
            if let Err(msg) = res {
                println!("split error: {msg}");
            }
        }
    }

    pub fn update(
        &mut self,
        ctx: &Context,
        grid: Rc<RefCell<dyn Grid>>,
        action_q: &mut VecDeque<Box<dyn Action>>,
    ) {
        if !self.is_showing {
            return;
        }

        let mut is_showing = self.is_showing;
        Window::new("Canvas Menu")
            .open(&mut is_showing)
            .show(ctx, |ui| self.draw(ui, grid, action_q));

        self.is_showing = is_showing;
    }
}

impl TopMenuBarItem for CanvasMenuUi {
    fn draw(&mut self, ui: &mut Ui) {
        if ui.button("Canvas").clicked() {
            self.is_showing = true;
        }
    }
}
