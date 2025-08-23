use egui::{Context, Ui, Window};

use crate::common::{binary_file_io, drawing::Drawing, ojd_file_codec};

use super::top_menu_bar_item::TopMenuBarItem;
use rfd::FileDialog;

pub struct FileMenuUi {
    is_showing: bool,
}

impl FileMenuUi {
    pub fn new() -> Self {
        FileMenuUi { is_showing: false }
    }

    pub fn draw(&mut self, ui: &mut Ui, drawing: &mut dyn Drawing) {
        let default_filename = "drawing.ojd";
        if ui.button("Save").clicked() {
            let mut encoded_binary = vec![];
            let res = ojd_file_codec::encode(drawing, &mut encoded_binary);
            if let Err(msg) = res {
                println!("{msg}");
                return;
            }

            let save_path = FileDialog::new()
                .set_file_name(default_filename)
                .set_directory("/")
                .save_file();

            if save_path.is_none() {
                // 保存せずにダイアログを閉じたりした場合なので、特に何もせずに返る
                return;
            }

            let res = binary_file_io::write_binary_file(
                save_path.unwrap().to_str().unwrap(),
                &encoded_binary,
            );
            if let Err(msg) = res {
                println!("{msg}");
                return;
            }
            println!("Saved");
        }

        if ui.button("Load").clicked() {
            let load_path = FileDialog::new()
                .add_filter("ojd", &["ojd"])
                .set_directory("/")
                .pick_file();

            if load_path.is_none() {
                // ファイルを選択せずにダイアログを閉じたりした場合なので、特に何もせずに返る
                return;
            }

            let res = binary_file_io::read_binary_file(load_path.unwrap().to_str().unwrap());
            if let Err(msg) = res {
                println!("{msg}");
                return;
            } else if let Ok(decoded_binary) = res {
                let res = ojd_file_codec::decode(&decoded_binary, drawing);
                if let Err(msg) = res {
                    println!("{msg}");
                    return;
                }
            }
            println!("Loaded");
        }
    }

    pub fn update(&mut self, ctx: &Context, drawing: &mut dyn Drawing) {
        if !self.is_showing {
            return;
        }

        let mut is_showing = self.is_showing;
        Window::new("Save Drawing Menu")
            .open(&mut is_showing)
            .show(ctx, |ui| self.draw(ui, drawing));
        self.is_showing = is_showing;
    }
}

impl TopMenuBarItem for FileMenuUi {
    fn draw(&mut self, ui: &mut egui::Ui) {
        if ui.button("File").clicked() {
            self.is_showing = true;
        }
    }
}
