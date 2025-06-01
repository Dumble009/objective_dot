use eframe::egui::*;

use crate::common::canvas_grid::Grid;

use super::top_menu_bar_item::TopMenuBarItem;

pub struct CanvasMenuUi {
    is_showing: bool,
}

impl CanvasMenuUi {
    pub fn new() -> Self {
        CanvasMenuUi { is_showing: false }
    }

    fn draw(&mut self, ui: &mut Ui, grid: &mut dyn Grid) {
        let mut layout = Layout::left_to_right(Align::Min);
        layout.main_wrap = true;

        ui.horizontal(|ui| {
            ui.label("Width "); // Height と文字数を揃えるためにスペースを入れている
            let mut width = grid.get_grid_width();
            ui.add(DragValue::new(&mut width));
            let res = grid.set_grid_width(width);
            if let Err(msg) = res {
                println!("set width error: {msg}");
            }
        });

        ui.horizontal(|ui| {
            ui.label("Height");
            let mut height = grid.get_grid_height();
            ui.add(DragValue::new(&mut height));
            let res = grid.set_grid_height(height);
            if let Err(msg) = res {
                println!("set height error: {msg}");
            }
        });

        if ui.button("split").clicked() {
            let res = grid.split();
            if let Err(msg) = res {
                println!("split error: {msg}");
            }
        }
    }

    pub fn update(&mut self, ctx: &Context, grid: &mut dyn Grid) {
        if !self.is_showing {
            return;
        }

        let mut is_showing = self.is_showing;
        Window::new("Canvas Menu")
            .open(&mut is_showing)
            .show(ctx, |ui| self.draw(ui, grid));

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
