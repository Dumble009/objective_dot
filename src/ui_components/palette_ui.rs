use crate::common::color::ODColor;
use crate::common::palette::Palette;
use crate::ui_components::color_picker_ui::{ColorPickMode, ColorPickResult, ColorPickerUi};
use eframe::egui::*;

use super::top_menu_bar_item::TopMenuBarItem;

pub struct PaletteUi {
    color_picker: ColorPickerUi,
    is_showing: bool,
}

impl PaletteUi {
    pub fn new() -> Self {
        PaletteUi {
            color_picker: ColorPickerUi::new(),
            is_showing: false,
        }
    }

    fn add_color(&mut self, color: ODColor, palette: &mut dyn Palette) -> Result<(), String> {
        palette.add_color(color)
    }

    fn draw(&mut self, ctx: &Context, ui: &mut Ui, palette: &mut dyn Palette) {
        if self.color_picker.is_showing() {
            self.color_picker.draw(ctx);
            let result = self.color_picker.get_color();

            if let ColorPickResult::AddNewColor(color) = result {
                self.add_color(color, palette).unwrap();
                self.color_picker.hide();
            } else if let ColorPickResult::ChangeColor(idx, color) = result {
                palette.change_color(idx, color).unwrap();
                self.color_picker.hide();
            } else if result == ColorPickResult::Canceled {
                self.color_picker.hide();
            }
        }

        if ui.button("Add Color").clicked() {
            self.color_picker
                .show("Add New Color", ColorPickMode::AddNewColor);
        }

        let mut layout = Layout::left_to_right(Align::Min);
        layout.main_wrap = true;
        ui.with_layout(layout, |ui| self.draw_color_boxes(ui, palette));
    }

    fn draw_color_boxes(&mut self, ui: &mut Ui, palette: &mut dyn Palette) {
        for idx in 0..palette.get_color_count() {
            let color_i = palette.get_color(idx).unwrap();
            let button = Button::new("").fill(color_i.to_color32());
            let button_add_res = ui.add(button);

            if button_add_res.clicked_by(PointerButton::Primary) {
                if let Err(msg) = palette.select_color(idx) {
                    println!("Palette Error: {msg}");
                }
            } else if button_add_res.clicked_by(PointerButton::Secondary) {
                self.color_picker
                    .show("Change Color", ColorPickMode::ChangeColor(idx));
            }
        }
    }

    pub fn update(&mut self, ctx: &Context, palette: &mut dyn Palette) {
        if !self.is_showing {
            return;
        }

        // open に直接 self.is_showing を渡すと、
        // show に渡しているクロージャが self.draw を保持しようとする際に
        // 2重 burrow になるというエラーが出るので、一時変数に入れている。
        let mut is_showing = self.is_showing;
        Window::new("Palette")
            .open(&mut is_showing)
            .show(ctx, |ui| self.draw(ctx, ui, palette));

        self.is_showing = is_showing;
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
