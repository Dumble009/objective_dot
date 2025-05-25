#![warn(clippy::all)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod common;
mod ui_components;

#[cfg(test)]
mod mock;

use common::drawing::{Drawing, ObjectDrawing};
use ui_components::canvas_menu_ui::CanvasMenuUi;
use ui_components::canvas_ui::*;
use ui_components::palette_ui::*;

use eframe::egui::*;
use ui_components::top_menu_bar_item::TopMenuBarItem;

pub struct ObjectiveDot {
    canvas_ui: CanvasUi,
    palette_ui: PaletteUi,
    canvas_menu_ui: CanvasMenuUi,
    drawing: ObjectDrawing,
}

impl ObjectiveDot {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        ObjectiveDot {
            canvas_ui: CanvasUi::new(),
            palette_ui: PaletteUi::new(),
            canvas_menu_ui: CanvasMenuUi::new(),
            drawing: ObjectDrawing::new(),
        }
    }
}

impl eframe::App for ObjectiveDot {
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {}
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        self.palette_ui.update(ctx, self.drawing.get_palette_mut());
        self.canvas_menu_ui.update(ctx, self.drawing.get_grid_mut());

        let top_menu_bar_items: Vec<&mut dyn TopMenuBarItem> =
            vec![&mut self.palette_ui, &mut self.canvas_menu_ui];
        self.canvas_ui
            .update(ctx, top_menu_bar_items, &mut self.drawing);
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
