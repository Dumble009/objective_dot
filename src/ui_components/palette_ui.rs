use crate::common::color::ODColor;
use crate::common::palette::Palette;
use color_picker::{color_picker_color32, Alpha};
use eframe::egui::*;

use super::top_menu_bar_item::TopMenuBarItem;

pub struct PaletteUi {
    palette: Palette,
    picked_color: Color32,
    is_showing: bool,
}

impl PaletteUi {
    pub fn new() -> Self {
        PaletteUi {
            palette: Palette::new(),
            picked_color: Color32::WHITE,
            is_showing: false,
        }
    }

    fn add_color(&mut self, color: ODColor) -> Result<(), String> {
        self.palette.add_color(color)
    }

    fn draw(&mut self, ui: &mut Ui) {
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

    pub fn update(&mut self, _ctx: &Context, _frame: &mut eframe::Frame) {
        if !self.is_showing {
            return;
        }

        // open に直接 self.is_showing を渡すと、
        // show に渡しているクロージャが self.draw を保持しようとする際に
        // 2重 burrow になるというエラーが出るので、一時変数に入れている。
        let mut is_showing = self.is_showing;
        Window::new("Palette")
            .open(&mut is_showing)
            .show(_ctx, |ui| self.draw(ui));

        self.is_showing = is_showing;
    }

    pub fn clone_palette(&self) -> Palette {
        self.palette.clone()
    }
}

impl TopMenuBarItem for PaletteUi {
    fn draw(&mut self, ui: &mut Ui) {
        if ui.button("Palette").clicked() {
            self.is_showing = true;
        }
    }
}

include!("tests/palette_ui_test.rs");
