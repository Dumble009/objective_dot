use crate::common::canvas_grid::Grid;
use crate::common::palette::Palette;
use eframe::egui::*;

use super::top_menu_bar_item::TopMenuBarItem;

const DEFAULT_SQUARE_WIDTH: u32 = 30;
const DEFAULT_SQUARE_HEIGHT: u32 = 30;
const TOP_MENU_BAR_HEIGHT: u32 = 20;

pub struct CanvasUi {}

impl CanvasUi {
    pub fn new() -> Self {
        CanvasUi {}
    }

    fn draw_grid(&self, ui: &mut Ui, grid: &dyn Grid, palette: &mut Palette) -> Result<(), String> {
        let mut square_x = 0;
        let mut square_y = 0;

        for y in 0..grid.get_grid_height() {
            for x in 0..grid.get_grid_width() {
                let color_idx = grid.get_color(x, y)?;
                let square_rect = Rect::from_min_max(
                    Pos2::new(square_x as f32, (square_y + TOP_MENU_BAR_HEIGHT) as f32),
                    Pos2::new(
                        (square_x + DEFAULT_SQUARE_WIDTH) as f32,
                        (square_y + DEFAULT_SQUARE_HEIGHT + TOP_MENU_BAR_HEIGHT) as f32,
                    ),
                );

                let color = palette.get_color(color_idx)?;
                let fill_color = color.to_color32();

                let stroke_color = Color32::from_rgb(
                    255 - fill_color.r(),
                    255 - fill_color.g(),
                    255 - fill_color.b(),
                );
                let grid_stroke = Stroke::new(1.0, stroke_color);

                ui.painter()
                    .rect(square_rect, 0, fill_color, grid_stroke, StrokeKind::Middle);

                square_x += DEFAULT_SQUARE_WIDTH;
            }
            square_x = 0;
            square_y += DEFAULT_SQUARE_HEIGHT;
        }

        Ok(())
    }

    fn get_grid_id_pair(&self, response: &Response) -> Result<(i32, i32), String> {
        // cursor_pos はウインドウの左上を (0, 0) とする座標系の値で返ってくる想定
        if let Some(cursor_pos) = response.interact_pointer_pos() {
            let grid_x = (cursor_pos.x / (DEFAULT_SQUARE_WIDTH as f32)) as i32;
            let grid_y = ((cursor_pos.y - TOP_MENU_BAR_HEIGHT as f32)
                / (DEFAULT_SQUARE_HEIGHT as f32)) as i32;

            if grid_x < 0
                || grid_x >= DEFAULT_SQUARE_WIDTH as i32
                || grid_y < 0
                || grid_y >= DEFAULT_SQUARE_HEIGHT as i32
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
        grid: &dyn Grid,
        palette: &mut Palette,
    ) -> Result<(), String> {
        let color_idx = grid.get_color(grid_x as usize, grid_y as usize)?;
        palette.select_color(color_idx)?;

        Ok(())
    }

    fn fill_by_cursor(
        &mut self,
        grid_x: i32,
        grid_y: i32,
        grid: &mut dyn Grid,
        palette: &mut Palette,
    ) -> Result<(), String> {
        grid.set_color(
            grid_x as usize,
            grid_y as usize,
            palette.get_current_selected_idx()?,
        )?;

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

    fn draw(&mut self, ui: &mut Ui, grid: &mut dyn Grid, palette: &mut Palette) {
        let (response, _) = ui.allocate_painter(
            ui.available_size_before_wrap(),
            Sense::drag() | Sense::click(),
        );

        if let Ok((grid_x, grid_y)) = self.get_grid_id_pair(&response) {
            if response.dragged_by(PointerButton::Primary) {
                if let Err(msg) = self.fill_by_cursor(grid_x, grid_y, grid, palette) {
                    println!("Error!: {msg}");
                }
            }

            if response.clicked_by(PointerButton::Secondary) {
                if let Err(msg) = self.choose_color_from_grid(grid_x, grid_y, grid, palette) {
                    println!("Error!: {msg}");
                }
            }
        }

        if let Err(msg) = self.draw_grid(ui, grid, palette) {
            println!("Error!: {msg}");
        }
    }

    pub fn update(
        &mut self,
        ctx: &Context,
        top_menu_bar_items: Vec<&mut dyn TopMenuBarItem>,
        grid: &mut dyn Grid,
        palette: &mut Palette,
    ) {
        TopBottomPanel::top("wrap_app_top_bar")
            .show(ctx, |ui| self.draw_top_menu_bar(ui, top_menu_bar_items));
        CentralPanel::default().show(ctx, |ui| self.draw(ui, grid, palette));
    }
}

include!("tests/canvas_ui_test.rs");
