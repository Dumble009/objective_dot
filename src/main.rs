#![warn(clippy::all)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod actions;
mod common;
mod ui_components;

#[cfg(test)]
mod mock;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use common::drawing::{Drawing, ObjectDrawing};
use eframe::egui::*;
use ui_components::canvas_menu_ui::CanvasMenuUi;
use ui_components::canvas_ui::*;
use ui_components::drawing_preview_ui::DrawingPreviewUi;
use ui_components::file_menu_ui::FileMenuUi;
use ui_components::layer_window_ui::LayerWindowUi;
use ui_components::palette_ui::*;
use ui_components::top_menu_bar_item::TopMenuBarItem;

use actions::action_runner::ActionRunner;

pub struct ObjectiveDot {
    canvas_ui: CanvasUi,
    palette_ui: PaletteUi,
    canvas_menu_ui: CanvasMenuUi,
    save_drawing_ui: FileMenuUi,
    drawing: Rc<RefCell<dyn Drawing>>,
    drawing_preview_ui: DrawingPreviewUi,
    layer_window_ui: LayerWindowUi,
    action_runner: ActionRunner,
}

impl ObjectiveDot {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        ObjectiveDot {
            canvas_ui: CanvasUi::new(),
            palette_ui: PaletteUi::new(),
            canvas_menu_ui: CanvasMenuUi::new(),
            save_drawing_ui: FileMenuUi::new(),
            drawing: Rc::new(RefCell::new(ObjectDrawing::new())),
            drawing_preview_ui: DrawingPreviewUi::new(),
            layer_window_ui: LayerWindowUi::new(),
            action_runner: ActionRunner::new(),
        }
    }
}

impl eframe::App for ObjectiveDot {
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {}
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        self.save_drawing_ui.update(ctx, self.drawing.clone());
        let mut action_q = VecDeque::new();
        self.palette_ui
            .update(ctx, self.drawing.borrow().get_palette(), &mut action_q);
        self.canvas_menu_ui
            .update(ctx, self.drawing.clone(), &mut action_q);
        self.drawing_preview_ui.update(ctx, &*self.drawing.borrow());
        self.layer_window_ui.update(ctx, &*self.drawing.borrow());

        let top_menu_bar_items: Vec<&mut dyn TopMenuBarItem> = vec![
            &mut self.save_drawing_ui,
            &mut self.palette_ui,
            &mut self.canvas_menu_ui,
            &mut self.drawing_preview_ui,
            &mut self.layer_window_ui,
        ];
        self.canvas_ui
            .update(ctx, top_menu_bar_items, self.drawing.clone(), &mut action_q);

        for action in action_q {
            if let Err(msg) = self.action_runner.run_action(action) {
                println!("run_action failed: {msg}");
            }
        }

        ctx.input(|i| {
            if i.modifiers.ctrl && i.key_pressed(Key::Z) {
                if let Err(msg) = self.action_runner.undo_action() {
                    println!("undo_action failed: {msg}");
                }
            }
        });
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
