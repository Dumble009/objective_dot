use crate::{common::drawing::Drawing, ui_components::top_menu_bar_item::TopMenuBarItem};
use eframe::egui::Ui;
use eframe::egui::*;

use std::cell::RefCell;
use std::rc::Rc;

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

    fn draw(&mut self, ui: &mut Ui, drawing: Rc<RefCell<dyn Drawing>>) {
        let layer_count = drawing.borrow().get_layer_count();
        ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
            for i in 0..layer_count {
                self.draw_item(ui, drawing.clone(), i);
            }

            if ui.button("Add Layer").clicked() {
                drawing.borrow_mut().add_grid_layer();
            }
        });
    }

    fn draw_item(&self, ui: &mut Ui, drawing: Rc<RefCell<dyn Drawing>>, layer_index: usize) {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label(format!("Layer {}", layer_index));
                if ui.button("Set Active").clicked() {
                    drawing
                        .borrow_mut()
                        .set_active_layer_idx(layer_index)
                        .unwrap();
                }
            });

            ui.vertical(|ui| {
                if ui.button("Up").clicked() {
                    println!("Move layer {} up", layer_index);
                }
                if ui.button("Down").clicked() {
                    println!("Move layer {} down", layer_index);
                }
            });

            const PREVIEW_SIZE: f32 = 64.0_f32;
            let (_, painter) =
                ui.allocate_painter(egui::vec2(PREVIEW_SIZE, PREVIEW_SIZE), egui::Sense::hover());

            let width = drawing.borrow().get_grid_width();
            let height = drawing.borrow().get_grid_height();

            let dot_width = PREVIEW_SIZE / width as f32;
            let dot_height = PREVIEW_SIZE / height as f32;

            for y in 0..height {
                for x in 0..width {
                    let color_idx = drawing
                        .borrow()
                        .get_grid_layer(layer_index)
                        .unwrap()
                        .borrow()
                        .get_color(x, y)
                        .unwrap_or(0);
                    let color = drawing
                        .borrow()
                        .get_palette()
                        .borrow()
                        .get_color(color_idx)
                        .unwrap_or_default();
                    let rect = Rect::from_min_size(
                        painter.clip_rect().min
                            + egui::vec2(x as f32 * dot_width, y as f32 * dot_height),
                        egui::vec2(dot_width, dot_height),
                    );
                    painter.rect_filled(rect, 0.0, color.to_color32());
                }
            }
        });
    }

    pub fn update(&mut self, ctx: &Context, drawing: Rc<RefCell<dyn Drawing>>) {
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
