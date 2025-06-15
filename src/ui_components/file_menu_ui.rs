use egui::{Context, Ui, Window};

use crate::common::{canvas_grid::Grid, palette::Palette};

use super::top_menu_bar_item::TopMenuBarItem;

pub struct FileMenuUi {
    is_showing: bool,
}

impl FileMenuUi {
    pub fn new() -> Self {
        FileMenuUi { is_showing: false }
    }

    pub fn draw(&mut self, ui: &mut Ui, grid: &dyn Grid, palette: &dyn Palette) {
        if ui.button("Save").clicked() {
            println!("Saved");
        }

        if ui.button("Load").clicked() {
            println!("Loaded");
        }
    }

    pub fn update(&mut self, ctx: &Context, grid: &dyn Grid, palette: &dyn Palette) {
        if !self.is_showing {
            return;
        }

        let mut is_showing = self.is_showing;
        Window::new("Save Drawing Menu")
            .open(&mut is_showing)
            .show(ctx, |ui| self.draw(ui, grid, palette));
        self.is_showing = is_showing;
    }
}

impl TopMenuBarItem for FileMenuUi {
    fn draw(&mut self, ui: &mut egui::Ui) {
        if ui.button("File").clicked() {
            self.is_showing = true;
        }
    }
}
