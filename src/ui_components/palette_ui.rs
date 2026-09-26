use crate::actions::action::Action;
use crate::actions::palette_color_add_action::PaletteColorAddAction;
use crate::actions::palette_color_change_action::PaletteColorChangeAction;
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
            palette.borrow_mut().override_by_color_sample(&colorset)?;
        }

        Ok(())
    }

    fn draw(
        &mut self,
        ctx: &Context,
        ui: &mut Ui,
        palette: Rc<RefCell<dyn Palette>>,
        action_q: &mut VecDeque<Box<dyn Action>>,
    ) {
        if self.color_picker.is_showing() {
            self.color_picker.draw(ctx);
            let result = self.color_picker.get_color();

            if let ColorPickResult::AddNewColor(color) = result {
                let action = PaletteColorAddAction::new(palette.clone(), color);
                action_q.push_front(Box::new(action));
                self.color_picker.hide();
            } else if let ColorPickResult::ChangeColor(idx, color) = result {
                let before_color = palette.borrow().get_color(idx).unwrap();
                let action =
                    PaletteColorChangeAction::new(palette.clone(), idx, before_color, color);
                action_q.push_front(Box::new(action));
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

        let layout = Layout::top_down(Align::Min);
        ui.with_layout(layout, |ui| self.draw_color_boxes(ui, palette.clone()));
    }

    fn draw_color_boxes(&mut self, ui: &mut Ui, palette: Rc<RefCell<dyn Palette>>) {
        let color_count = palette.borrow().get_color_count();
        for idx in 0..color_count {
            self.draw_color_rows(ui, palette.clone(), idx);
        }
    }

    fn draw_color_rows(&mut self, ui: &mut Ui, palette: Rc<RefCell<dyn Palette>>, idx: usize) {
        let width = ui.available_width();
        const ROW_HEIGHT: f32 = 20.0;
        const COLOR_BUTTON_WIDTH: f32 = 20.0;
        const ADD_BRIGHTNESS_BUTTON_WIDTH: f32 = 20.0;

        let (row_rect, _) =
            ui.allocate_exact_size(egui::vec2(width, ROW_HEIGHT), egui::Sense::hover());
        if idx != 0 {
            // 左端の + ボタン
            let rect = egui::Rect::from_min_size(
                row_rect.left_top(),
                egui::vec2(ADD_BRIGHTNESS_BUTTON_WIDTH, ROW_HEIGHT),
            );
            let left_plus_button_res = ui.put(rect, egui::Button::new("+"));
            if left_plus_button_res.clicked_by(PointerButton::Primary) {
                if let Err(msg) = palette.borrow_mut().add_darkness_range(idx) {
                    println!("Palette Error: {msg}");
                }
            }

            // 右端の + ボタン
            let rect = egui::Rect::from_min_size(
                row_rect.right_top() - egui::vec2(ADD_BRIGHTNESS_BUTTON_WIDTH, 0.0),
                egui::vec2(ADD_BRIGHTNESS_BUTTON_WIDTH, ROW_HEIGHT),
            );
            let right_plus_button_res = ui.put(rect, egui::Button::new("+"));
            if right_plus_button_res.clicked_by(PointerButton::Primary) {
                if let Err(msg) = palette.borrow_mut().add_brightness_range(idx) {
                    println!("Palette Error: {msg}");
                }
            }
        }

        // 中央の色ボタン
        let color = palette.borrow().get_color((idx, 0).into()).unwrap();
        let button = Button::new("").fill(color.to_color32());

        let rect = egui::Rect::from_min_size(
            row_rect.center() - egui::vec2(COLOR_BUTTON_WIDTH / 2.0, ROW_HEIGHT / 2.0),
            egui::vec2(COLOR_BUTTON_WIDTH, ROW_HEIGHT),
        );
        let color_button_res = ui.put(rect, button);

        if color_button_res.clicked_by(PointerButton::Primary) {
            if let Err(msg) = palette.borrow_mut().select_color((idx, 0).into()) {
                println!("Palette Error: {msg}");
            }
        } else if color_button_res.clicked_by(PointerButton::Secondary) {
            self.color_picker
                .show("Change Color", ColorPickMode::ChangeColor((idx, 0).into()));
        }

        // 明るい側の色ボタン
        let brightness_range = palette.borrow().get_active_brightness_range(idx).unwrap();
        for brightness in 1..=brightness_range {
            let color = palette
                .borrow()
                .get_color((idx, brightness as i32).into())
                .unwrap();
            let button = Button::new("").fill(color.to_color32());

            let rect = egui::Rect::from_min_size(
                row_rect.center()
                    + egui::vec2(
                        COLOR_BUTTON_WIDTH / 2.0 + (brightness - 1) as f32 * COLOR_BUTTON_WIDTH,
                        -ROW_HEIGHT / 2.0,
                    ),
                egui::vec2(COLOR_BUTTON_WIDTH, ROW_HEIGHT),
            );
            let color_button_res = ui.put(rect, button);

            if color_button_res.clicked_by(PointerButton::Primary) {
                if let Err(msg) = palette
                    .borrow_mut()
                    .select_color((idx, brightness as i32).into())
                {
                    println!("Palette Error: {msg}");
                }
            } else if color_button_res.clicked_by(PointerButton::Secondary) {
                self.color_picker.show(
                    "Change Color",
                    ColorPickMode::ChangeColor((idx, brightness as i32).into()),
                );
            }
        }

        // 暗い側の色ボタン
        let darkness_range = palette.borrow().get_active_darkness_range(idx).unwrap();
        for darkness in 1..=darkness_range {
            let color = palette
                .borrow()
                .get_color((idx, -(darkness as i32)).into())
                .unwrap();
            let button = Button::new("").fill(color.to_color32());
            let rect = egui::Rect::from_min_size(
                row_rect.center()
                    + egui::vec2(
                        -COLOR_BUTTON_WIDTH / 2.0 - (darkness) as f32 * COLOR_BUTTON_WIDTH,
                        -ROW_HEIGHT / 2.0,
                    ),
                egui::vec2(COLOR_BUTTON_WIDTH, ROW_HEIGHT),
            );
            let color_button_res = ui.put(rect, button);

            if color_button_res.clicked_by(PointerButton::Primary) {
                if let Err(msg) = palette
                    .borrow_mut()
                    .select_color((idx, -(darkness as i32)).into())
                {
                    println!("Palette Error: {msg}");
                }
            } else if color_button_res.clicked_by(PointerButton::Secondary) {
                self.color_picker.show(
                    "Change Color",
                    ColorPickMode::ChangeColor((idx, -(darkness as i32)).into()),
                );
            }
        }
    }

    pub fn update(
        &mut self,
        ctx: &Context,
        palette: Rc<RefCell<dyn Palette>>,
        action_q: &mut VecDeque<Box<dyn Action>>,
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
