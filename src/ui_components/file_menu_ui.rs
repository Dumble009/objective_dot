use egui::{Context, DragValue, Ui, Window};

use crate::common::{binary_file_io, drawing::Drawing, ojd_file_codec, png_write};

use super::top_menu_bar_item::TopMenuBarItem;
use rfd::FileDialog;

pub struct FileMenuUi {
    is_showing: bool,
    pixel_per_dot_for_png_export: usize,
}

impl FileMenuUi {
    pub fn new() -> Self {
        FileMenuUi {
            is_showing: false,
            pixel_per_dot_for_png_export: 100,
        }
    }

    fn save(&self, drawing: &mut dyn Drawing) -> Result<(), String> {
        let mut encoded_binary = vec![];
        ojd_file_codec::encode(drawing, &mut encoded_binary)?;

        let default_filename = "drawing.ojd";
        let save_path = FileDialog::new()
            .set_file_name(default_filename)
            .set_directory("/")
            .save_file();

        if save_path.is_none() {
            // 保存せずにダイアログを閉じたりした場合なので、特に何もせずに返る
            return Ok(());
        }

        binary_file_io::write_binary_file(save_path.unwrap().to_str().unwrap(), &encoded_binary)?;
        println!("Saved");
        Ok(())
    }

    fn load(&self, drawing: &mut dyn Drawing) -> Result<(), String> {
        let load_path = FileDialog::new()
            .add_filter("ojd", &["ojd"])
            .set_directory("/")
            .pick_file();

        if load_path.is_none() {
            // ファイルを選択せずにダイアログを閉じたりした場合なので、特に何もせずに返る
            return Ok(());
        }

        let decoded_binary =
            binary_file_io::read_binary_file(load_path.unwrap().to_str().unwrap())?;
        ojd_file_codec::decode(&decoded_binary, drawing)?;
        println!("Loaded");
        Ok(())
    }

    fn export_as_png(&self, drawing: &mut dyn Drawing, pixel_per_dot: usize) -> Result<(), String> {
        let export_path = FileDialog::new()
            .add_filter("png", &["png"])
            .set_directory("/")
            .save_file();

        if export_path.is_none() {
            // ファイルを選択せずにダイアログを閉じたりした場合なので、特に何もせずに返る
            return Ok(());
        }

        let bitmap = crate::common::bitmap::Bitmap::from_drawing(drawing, pixel_per_dot)?;
        png_write::write_png(&bitmap, export_path.unwrap().to_str().unwrap())?;
        println!("Exported as PNG");
        Ok(())
    }

    pub fn draw(&mut self, ui: &mut Ui, drawing: &mut dyn Drawing) {
        if ui.button("Save").clicked() {
            let res = self.save(drawing);
            if let Err(msg) = res {
                println!("Save Drawing Error : {msg}");
            }
        }

        if ui.button("Load").clicked() {
            let res = self.load(drawing);
            if let Err(msg) = res {
                println!("Load Drawing Error : {msg}");
            }
        }

        if ui.button("Export as PNG").clicked() {
            let res = self.export_as_png(drawing, self.pixel_per_dot_for_png_export);
            if let Err(msg) = res {
                println!("Export as PNG Error : {msg}");
            }
        }
        ui.horizontal(|ui| {
            ui.label("Ratio : ");
            ui.add(
                DragValue::new(&mut self.pixel_per_dot_for_png_export).update_while_editing(false),
            );
        });
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
