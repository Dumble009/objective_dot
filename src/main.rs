#![warn(clippy::all)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod common;
mod ui_components;

use ui_components::canvas_ui::*;
use ui_components::palette_ui::*;

use eframe::egui::*;
use ui_components::top_menu_bar_item::TopMenuBarItem;

pub struct ObjectiveDot {
    canvas_ui: CanvasUi,
    palette_ui: PaletteUi,
}

impl ObjectiveDot {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        ObjectiveDot {
            canvas_ui: CanvasUi::new(),
            palette_ui: PaletteUi::new(),
        }
    }
}

impl eframe::App for ObjectiveDot {
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {}
    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        self.palette_ui.update(ctx, frame);

        let palette = self.palette_ui.clone_palette();
        self.canvas_ui.set_palette(palette);

        let top_menu_bar_items: Vec<&mut dyn TopMenuBarItem> = vec![&mut self.palette_ui];
        self.canvas_ui.update(ctx, top_menu_bar_items);
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "objective dot",
        options,
        Box::new(|cc| Ok(Box::new(ObjectiveDot::new(cc)))),
    )
}
