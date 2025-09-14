use crate::actions::palette_color_change_action::PaletteColorChangeAction;
use crate::common::color::ODColor;
use crate::common::paint_net_codec::decode;
use crate::common::palette::Palette;
use crate::ui_components::color_picker_ui::{ColorPickMode, ColorPickResult, ColorPickerUi};
use eframe::egui::*;
use rfd::FileDialog;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::fs;
use std::rc::Rc;

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

    fn add_color(
        &mut self,
        color: ODColor,
        palette: Rc<RefCell<dyn Palette>>,
    ) -> Result<(), String> {
        palette.borrow_mut().add_color(color)
    }

    fn load_color(&mut self, palette: Rc<RefCell<dyn Palette>>) -> Result<(), String> {
        let palette_file = FileDialog::new()
            .add_filter("text", &["txt"])
            .set_directory("/")
            .pick_file()
            .unwrap_or_default();

        let palette_file_path_str = palette_file.to_str().unwrap_or("");

        if palette_file_path_str.is_empty() {
            // ファイルを選ばずにダイアログを閉じた場合などもあるので、エラーにはしない
            return Ok(());
        }

        let content = fs::read_to_string(palette_file_path_str);
        if let Err(err) = content {
            return Err(err.to_string());
        } else if let Ok(content) = content {
            let mut colorset = vec![];
            decode(content, &mut colorset)?;
            palette.borrow_mut().override_by_colorset(&colorset)?;
        }

        Ok(())
    }

    fn draw(
        &mut self,
        ctx: &Context,
        ui: &mut Ui,
        palette: Rc<RefCell<dyn Palette>>,
        action_q: &mut VecDeque<PaletteColorChangeAction>,
    ) {
        if self.color_picker.is_showing() {
            self.color_picker.draw(ctx);
            let result = self.color_picker.get_color();

            if let ColorPickResult::AddNewColor(color) = result {
                self.add_color(color, palette.clone()).unwrap();
                self.color_picker.hide();
            } else if let ColorPickResult::ChangeColor(idx, color) = result {
                let before_color = palette.borrow().get_color(idx).unwrap();
                let action =
                    PaletteColorChangeAction::new(palette.clone(), idx, before_color, color);
                action_q.push_front(action);
                // palette.borrow_mut().change_color(idx, color).unwrap();
                self.color_picker.hide();
            } else if result == ColorPickResult::Canceled {
                self.color_picker.hide();
            }
        }

        if ui.button("Add Color").clicked() {
            self.color_picker
                .show("Add New Color", ColorPickMode::AddNewColor);
        }

        if ui.button("Load Palette").clicked() {
            let res = self.load_color(palette.clone());
            if let Err(err) = res {
                println!("filed to load_color. {err}");
                return;
            }
        }

        let mut layout = Layout::left_to_right(Align::Min);
        layout.main_wrap = true;
        ui.with_layout(layout, |ui| self.draw_color_boxes(ui, palette.clone()));
    }

    fn draw_color_boxes(&mut self, ui: &mut Ui, palette: Rc<RefCell<dyn Palette>>) {
        let color_count = palette.borrow().get_color_count();
        for idx in 0..color_count {
            let color_i = palette.borrow().get_color(idx).unwrap();
            let button = Button::new("").fill(color_i.to_color32());
            let button_add_res = ui.add(button);

            if button_add_res.clicked_by(PointerButton::Primary) {
                if let Err(msg) = palette.borrow_mut().select_color(idx) {
                    println!("Palette Error: {msg}");
                }
            } else if button_add_res.clicked_by(PointerButton::Secondary) {
                self.color_picker
                    .show("Change Color", ColorPickMode::ChangeColor(idx));
            }
        }
    }

    pub fn update(
        &mut self,
        ctx: &Context,
        palette: Rc<RefCell<dyn Palette>>,
        action_q: &mut VecDeque<PaletteColorChangeAction>,
    ) {
        if !self.is_showing {
            return;
        }

        // open に直接 self.is_showing を渡すと、
        // show に渡しているクロージャが self.draw を保持しようとする際に
        // 2重 burrow になるというエラーが出るので、一時変数に入れている。
        let mut is_showing = self.is_showing;
        Window::new("Palette")
            .open(&mut is_showing)
            .show(ctx, |ui| self.draw(ctx, ui, palette, action_q));

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
