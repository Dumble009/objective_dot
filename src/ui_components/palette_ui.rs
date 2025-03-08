use crate::common::color::ODColor;
use crate::common::palette::Palette;
use color_picker::{color_picker_color32, Alpha};
use eframe::egui::*;

pub struct PaletteUi {
    palette: Palette,
    picked_color: Color32,
}

impl PaletteUi {
    pub fn new() -> Self {
        PaletteUi {
            palette: Palette::new(),
            picked_color: Color32::WHITE,
        }
    }

    fn add_color(&mut self, color: ODColor) -> Result<(), String> {
        self.palette.add_color(color)
    }

    fn update(&mut self, ui: &mut Ui) {
        let alpha = Alpha::Opaque;
        color_picker_color32(ui, &mut self.picked_color, alpha);
        if ui.button("Add Color").clicked() {
            self.add_color(ODColor::from_color32(self.picked_color))
                .unwrap_or_default();
        }

        let mut layout = Layout::left_to_right(Align::Min);
        layout.main_wrap = true;
        ui.with_layout(layout, |ui| self.draw_color_boxes(ui));
    }

    fn draw_color_boxes(&mut self, ui: &mut Ui) {
        for idx in 0..self.palette.get_color_count() {
            let color_i = self.palette.get_color(idx).unwrap();
            let button = Button::new("").fill(color_i.to_color32());

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
