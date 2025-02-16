#![warn(clippy::all)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui::*;

#[derive(Default)]
pub struct ObjectiveDot {}

impl ObjectiveDot {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for ObjectiveDot {
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {}
    fn update(&mut self, _ctx: &Context, _frame: &mut eframe::Frame) {}
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "objective dot",
        options,
        Box::new(|cc| Ok(Box::new(ObjectiveDot::new(cc)))),
    )
}
