use crate::common::color::ODColor;
use crate::common::palette::{Palette, PaletteColorIndex};
use eframe::egui::*;

pub struct PaletteUi {
    palette: Palette,
}

impl PaletteUi {
    pub fn new() -> Self {
        PaletteUi {
            palette: Palette::new(),
        }
    }

    fn add_color(&mut self, color: ODColor) -> Result<(), String> {
        self.palette.add_color(color)
    }

    fn update(&mut self, ui: &mut Ui) {
        if ui.button("Add Color").clicked() {
            self.add_color(ODColor::new(255, 0, 0)).unwrap_or_default();
        }

        let mut layout = Layout::left_to_right(Align::Min);
        layout.main_wrap = true;
        ui.with_layout(layout, |ui| self.draw_color_boxes(ui));
    }

    fn draw_color_boxes(&mut self, ui: &mut Ui) {
        for idx in 0..self.palette.get_color_count() {
            let button = Button::new("").fill(Color32::RED);

            if ui.add(button).clicked() {
                if let Err(msg) = self.palette.select_color(idx) {
                    println!("Palette Error: {msg}");
                }
            }
        }
    }

    pub fn clone_palette(&self) -> Palette {
        self.palette.clone()
    }
}

impl eframe::App for PaletteUi {
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {}
    fn update(&mut self, _ctx: &Context, _frame: &mut eframe::Frame) {
        Window::new("Palette").show(_ctx, |ui| self.update(ui));
    }
}

include!("tests/palette_ui_test.rs");
