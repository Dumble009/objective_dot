use crate::common::drawing::Drawing;
use eframe::egui::*;

use super::top_menu_bar_item::TopMenuBarItem;

const DEFAULT_SQUARE_SIZE: f32 = 30.0;
const TOP_MENU_BAR_HEIGHT: u32 = 20;
const SCROLL_MAGNI: f32 = 0.1;

pub struct CanvasUi {
    square_root_pos: Pos2,
    square_size: f32,
}

impl CanvasUi {
    pub fn new() -> Self {
        CanvasUi {
            square_root_pos: Pos2::new(0.0, 0.0),
            square_size: DEFAULT_SQUARE_SIZE,
        }
    }

    fn draw_grid(&self, ui: &mut Ui, drawing: &dyn Drawing) -> Result<(), String> {
        let grid_height = drawing.get_grid().get_grid_height();
        let grid_width = drawing.get_grid().get_grid_width();

        for y in 0..grid_height {
            for x in 0..grid_width {
                let color_idx = drawing.get_grid().get_color(x, y)?;
                let square_pos = self.square_root_pos
                    + Vec2::new(
                        x as f32 * self.square_size,
                        y as f32 * self.square_size + TOP_MENU_BAR_HEIGHT as f32,
                    );
                let square_rect = Rect::from_min_max(
                    square_pos,
                    square_pos + Vec2::new(self.square_size, self.square_size),
                );

                let color = drawing.get_palette().get_color(color_idx)?;
                let fill_color = color.to_color32();

                let stroke_color = Color32::from_rgb(
                    255 - fill_color.r(),
                    255 - fill_color.g(),
                    255 - fill_color.b(),
                );
                let grid_stroke = Stroke::new(1.0, stroke_color);

                ui.painter()
                    .rect(square_rect, 0, fill_color, grid_stroke, StrokeKind::Middle);
            }
        }

        Ok(())
    }

    fn get_grid_id_pair(&self, response: &Response) -> Result<(i32, i32), String> {
        // cursor_pos はウインドウの左上を (0, 0) とする座標系の値で返ってくる想定
        if let Some(cursor_pos) = response.interact_pointer_pos() {
            let absolute_cursor_pos = cursor_pos - self.square_root_pos;

            let grid_x = (absolute_cursor_pos.x / (self.square_size)) as i32;
            let grid_y =
                ((absolute_cursor_pos.y - TOP_MENU_BAR_HEIGHT as f32) / self.square_size) as i32;

            if grid_x < 0
                || grid_x >= self.square_size as i32
                || grid_y < 0
                || grid_y >= self.square_size as i32
            {
                return Err(String::from("get_grid_id_pair : Invalid Position"));
            }

            return Ok((grid_x, grid_y));
        }

        Err(String::from("get_grid_id_pair : Invalid Pointer"))
    }

    fn choose_color_from_grid(
        &self,
        grid_x: i32,
        grid_y: i32,
        drawing: &mut dyn Drawing,
    ) -> Result<(), String> {
        let color_idx = drawing
            .get_grid()
            .get_color(grid_x as usize, grid_y as usize)?;
        drawing.get_palette_mut().select_color(color_idx)?;

        Ok(())
    }

    fn fill_by_cursor(
        &mut self,
        grid_x: i32,
        grid_y: i32,
        drawing: &mut dyn Drawing,
    ) -> Result<(), String> {
        let selected_idx = drawing.get_palette_mut().get_current_selected_idx()?;
        drawing
            .get_grid_mut()
            .set_color(grid_x as usize, grid_y as usize, selected_idx)?;

        Ok(())
    }

    fn draw_top_menu_bar(&self, ui: &mut Ui, top_menu_bar_items: Vec<&mut dyn TopMenuBarItem>) {
        let mut layout = Layout::left_to_right(Align::Min);
        layout.main_wrap = true;
        ui.with_layout(layout, |ui| {
            for top_menu_bar_item in top_menu_bar_items {
                top_menu_bar_item.draw(ui);
            }
        });
    }

    fn move_canvas(&mut self, response: &Response) {
        let mov = response.drag_delta();
        self.square_root_pos += mov;
        self.square_root_pos.x = if self.square_root_pos.x > 0.0 {
            0.0
        } else {
            self.square_root_pos.x
        };

        self.square_root_pos.y = if self.square_root_pos.y > 0.0 {
            0.0
        } else {
            self.square_root_pos.y
        };
    }

    fn zoom(&mut self, scroll_y: f32) {
        self.square_size += scroll_y * SCROLL_MAGNI;
    }

    fn draw(&mut self, ui: &mut Ui, ctx: &Context, drawing: &mut dyn Drawing) {
        let (response, _) = ui.allocate_painter(
            ui.available_size_before_wrap(),
            Sense::drag() | Sense::click() | Sense::hover(),
        );

        if let Ok((grid_x, grid_y)) = self.get_grid_id_pair(&response) {
            if response.dragged_by(PointerButton::Primary) {
                if let Err(msg) = self.fill_by_cursor(grid_x, grid_y, drawing) {
                    println!("Error!: {msg}");
                }
            }

            if response.clicked_by(PointerButton::Secondary) {
                if let Err(msg) = self.choose_color_from_grid(grid_x, grid_y, drawing) {
                    println!("Error!: {msg}");
                }
            }
        }

        if response.dragged_by(PointerButton::Middle) {
            self.move_canvas(&response);
        }

        if response.hovered() {
            let scroll = ctx.input(|i| i.raw_scroll_delta);
            self.zoom(scroll.y);
        }

        if let Err(msg) = self.draw_grid(ui, drawing) {
            println!("Error!: {msg}");
        }
    }

    pub fn update(
        &mut self,
        ctx: &Context,
        top_menu_bar_items: Vec<&mut dyn TopMenuBarItem>,
        drawing: &mut dyn Drawing,
    ) {
        TopBottomPanel::top("wrap_app_top_bar")
            .show(ctx, |ui| self.draw_top_menu_bar(ui, top_menu_bar_items));
        CentralPanel::default().show(ctx, |ui| self.draw(ui, ctx, drawing));
    }
}

include!("tests/canvas_ui_test.rs");
