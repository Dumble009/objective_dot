use eframe::egui::Ui;
pub trait TopMenuBarItem {
    fn draw(&mut self, ui: &mut Ui);
}
