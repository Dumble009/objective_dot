use crate::{
    common::{color::ODColor, drawing::Drawing},
    ui_components::top_menu_bar_item::TopMenuBarItem,
};
use eframe::egui::Ui;
use eframe::egui::*;

pub struct DrawingPreviewUi {
    is_showing: bool,
    window_size: Vec2,
}

impl DrawingPreviewUi {
    pub fn new() -> Self {
        DrawingPreviewUi {
            is_showing: false,
            window_size: Vec2::new(200.0, 200.0),
        }
    }

    fn draw(&mut self, ui: &mut Ui, drawing: &dyn Drawing) {
        let _ = ui.allocate_space(ui.available_size());

        let window_rect = ui.max_rect();
        let grid_w = drawing.get_grid().borrow().get_grid_width();
        let grid_h = drawing.get_grid().borrow().get_grid_height();

        let (pos, cell_size) = self.calc_drawing_element(window_rect, grid_w, grid_h);

        for y in 0..grid_h {
            for x in 0..grid_w {
                let color_idx = drawing.get_grid().borrow().get_color(x, y).unwrap_or(0);
                let color = drawing
                    .get_palette()
                    .borrow()
                    .get_color(color_idx)
                    .unwrap_or(ODColor::default());
                let rect = Rect::from_min_size(
                    pos + Vec2::new(x as f32 * cell_size as f32, y as f32 * cell_size as f32),
                    Vec2::new(cell_size as f32, cell_size as f32),
                );
                ui.painter().rect_filled(rect, 0.0, color.to_color32());
            }
        }

        self.window_size = window_rect.size();
    }

    fn calc_drawing_element(
        &self,
        window_rect: Rect,
        grid_w: usize,
        grid_h: usize,
    ) -> (Pos2, usize) {
        let window_x = window_rect.left_top().x;
        let window_y = window_rect.left_top().y;
        let window_w = window_rect.width();
        let window_h = window_rect.height();

        let cell_w = window_w / grid_w as f32;
        let cell_h = window_h / grid_h as f32;
        let cell_size = cell_w.min(cell_h);

        let x_space = (window_w - cell_size * grid_w as f32) / 2.0;
        let y_space = (window_h - cell_size * grid_h as f32) / 2.0;

        let start_x = window_x + x_space;
        let start_y = window_y + y_space;
        (Pos2::new(start_x, start_y), cell_size as usize)
    }

    pub fn update(&mut self, ctx: &Context, drawing: &dyn Drawing) {
        if !self.is_showing {
            return;
        }

        let mut is_showing = self.is_showing;
        Window::new("Preview")
            .open(&mut is_showing)
            .resizable(true)
            .default_size(self.window_size)
            .show(ctx, |ui| self.draw(ui, drawing));
        self.is_showing = is_showing;
    }
}

impl TopMenuBarItem for DrawingPreviewUi {
    fn draw(&mut self, ui: &mut Ui) {
        if ui.button("Preview").clicked() {
            self.is_showing = true;
        }
    }
}

include!("tests/drawing_preview_ui_test.rs");
