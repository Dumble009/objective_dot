use crate::common::color::ODColor;
use color_picker::{color_picker_color32, Alpha};
use eframe::egui::*;

pub struct ColorPickerUi {
    window_title: String,
    pick_result: ColorPickResult,
    pick_mode: ColorPickMode,
    pick_color: Color32,
    is_showing: bool,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum ColorPickResult {
    AddNewColor(ODColor),
    ChangeColor(usize, ODColor),
    Waiting,
    Canceled,
}

#[derive(PartialEq, Clone, Copy, Debug)]

pub enum ColorPickMode {
    AddNewColor,
    ChangeColor(usize),
}

impl ColorPickerUi {
    pub fn new() -> Self {
        ColorPickerUi {
            window_title: String::from(""),
            pick_result: ColorPickResult::Waiting,
            pick_mode: ColorPickMode::AddNewColor,
            pick_color: Color32::WHITE,
            is_showing: false,
        }
    }

    pub fn draw(&mut self, ctx: &Context) {
        let mut is_showing = self.is_showing;
        let window_title = self.window_title.clone();

        Window::new(&window_title)
            .open(&mut is_showing)
            .show(ctx, |ui| {
                self.draw_ui(ui);
            });

        self.is_showing = is_showing;
    }

    pub fn get_color(&self) -> ColorPickResult {
        self.pick_result
    }

    fn set_color(&mut self, color: Color32) {
        if self.pick_mode == ColorPickMode::AddNewColor {
            self.pick_result = ColorPickResult::AddNewColor(ODColor::from_color32(color));
        } else if let ColorPickMode::ChangeColor(idx) = self.pick_mode {
            self.pick_result = ColorPickResult::ChangeColor(idx, ODColor::from_color32(color));
        }
    }

    fn cancel(&mut self) {
        self.pick_result = ColorPickResult::Canceled;
    }

    fn draw_ui(&mut self, ui: &mut Ui) {
        let alpha = Alpha::Opaque;
        color_picker_color32(ui, &mut self.pick_color, alpha);

        if ui.button("Confirm").clicked() {
            self.set_color(self.pick_color);
        } else if ui.button("Cancel").clicked() {
            self.cancel();
        }
    }

    pub fn is_showing(&self) -> bool {
        self.is_showing
    }

    pub fn show(&mut self, title: &str, mode: ColorPickMode) {
        if self.is_showing {
            return;
        }
        self.window_title = String::from(title);
        self.pick_result = ColorPickResult::Waiting;
        self.pick_mode = mode;
        self.is_showing = true;
    }

    pub fn hide(&mut self) {
        self.is_showing = false;
    }
}

include!("tests/color_picker_ui_test.rs");
