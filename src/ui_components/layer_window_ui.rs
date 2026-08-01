use crate::{common::drawing::Drawing, ui_components::top_menu_bar_item::TopMenuBarItem};
use eframe::egui::Ui;
use eframe::egui::*;

pub struct LayerWindowUi {
    is_showing: bool,
    window_size: Vec2,
}

impl LayerWindowUi {
    pub fn new() -> Self {
        LayerWindowUi {
            is_showing: false,
            window_size: Vec2::new(200.0, 200.0),
        }
    }

    fn draw(&mut self, ui: &mut Ui, _drawing: &dyn Drawing) {
        ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
            for i in 0..10 {
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(format!("Layer {}", i));
                        ui.vertical(|ui| {
                            if ui.button("Up").clicked() {
                                println!("Move layer {} up", i);
                            }
                            if ui.button("Down").clicked() {
                                println!("Move layer {} down", i);
                            }
                        })
                    });
                });
            }
        });
    }

    pub fn update(&mut self, ctx: &Context, drawing: &dyn Drawing) {
        if !self.is_showing {
            return;
        }

        let mut is_showing = self.is_showing;
        Window::new("Layers")
            .open(&mut is_showing)
            .resizable(true)
            .default_size(self.window_size)
            .show(ctx, |ui| self.draw(ui, drawing));
        self.is_showing = is_showing;
    }
}

impl TopMenuBarItem for LayerWindowUi {
    fn draw(&mut self, ui: &mut Ui) {
        if ui.button("Layers").clicked() {
            self.is_showing = true;
        }
    }
}
