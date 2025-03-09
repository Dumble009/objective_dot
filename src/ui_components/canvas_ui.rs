use crate::common::palette::{Palette, PaletteColorIndex};
use eframe::egui::*;

use super::top_menu_bar_item::{self, TopMenuBarItem};

const GRID_WIDTH: u32 = 10;
const GRID_HEIGHT: u32 = 10;
const DEFAULT_SQUARE_WIDTH: u32 = 30;
const DEFAULT_SQUARE_HEIGHT: u32 = 30;
const TOP_MENU_BAR_HEIGHT: u32 = 20;

#[derive(Default)]
struct CanvasGrid {
    grid: Vec<Vec<PaletteColorIndex>>,
}

impl CanvasGrid {
    fn new() -> Self {
        CanvasGrid {
            grid: vec![vec![0; GRID_WIDTH as usize]; GRID_HEIGHT as usize],
        }
    }

    fn coordinate_validation(&self, x: usize, y: usize) -> Result<(), String> {
        if y < self.grid.len() && x < self.grid[y].len() {
            Ok(())
        } else {
            Err(format!("args are out of range! x={x}, y={y}"))
        }
    }

    fn set_color(&mut self, x: usize, y: usize, val: PaletteColorIndex) -> Result<(), String> {
        self.coordinate_validation(x, y)?;

        self.grid[y][x] = val;

        Ok(())
    }

    fn get_color(&self, x: usize, y: usize) -> Result<PaletteColorIndex, String> {
        self.coordinate_validation(x, y)?;
        Ok(self.grid[y][x])
    }
}

pub struct CanvasUi {
    grid: CanvasGrid,
    palette: Palette,
}

impl CanvasUi {
    pub fn new() -> Self {
        CanvasUi {
            grid: CanvasGrid::new(),
            palette: Palette::new(),
        }
    }

    fn draw_grid(&self, ui: &mut Ui) -> Result<(), String> {
        let mut square_x = 0;
        let mut square_y = 0;

        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                let color_idx = self.grid.get_color(x as usize, y as usize)?;
                let square_rect = Rect::from_min_max(
                    Pos2::new(square_x as f32, (square_y + TOP_MENU_BAR_HEIGHT) as f32),
                    Pos2::new(
                        (square_x + DEFAULT_SQUARE_WIDTH) as f32,
                        (square_y + DEFAULT_SQUARE_HEIGHT + TOP_MENU_BAR_HEIGHT) as f32,
                    ),
                );

                let color = self.palette.get_color(color_idx)?;
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

    fn fill_by_cursor(&mut self, ui: &mut Ui) -> Result<(), String> {
        let (response, _) = ui.allocate_painter(ui.available_size_before_wrap(), Sense::drag());

        // cursor_pos はウインドウの左上を (0, 0) とする座標系の値で返ってくる想定
        if let Some(cursor_pos) = response.interact_pointer_pos() {
            let grid_x = (cursor_pos.x / (DEFAULT_SQUARE_WIDTH as f32)) as i32;
            let grid_y = ((cursor_pos.y - TOP_MENU_BAR_HEIGHT as f32)
                / (DEFAULT_SQUARE_HEIGHT as f32)) as i32;
            if !(0 <= grid_x
                && grid_x < GRID_WIDTH as i32
                && 0 <= grid_y
                && grid_y < GRID_HEIGHT as i32)
            {
                return Ok(());
            }

            self.grid.set_color(
                grid_x as usize,
                grid_y as usize,
                self.palette.get_current_active_idx()?,
            )?;
        }
        Ok(())
    }

    fn draw_top_menu_bar(&self, ui: &mut Ui, top_menu_bar_items: Vec<&mut dyn TopMenuBarItem>) {
        for top_menu_bar_item in top_menu_bar_items {
            top_menu_bar_item.draw(ui);
        }
    }

    fn draw(&mut self, ui: &mut Ui) {
        if let Err(msg) = self.fill_by_cursor(ui) {
            println!("Error!: {msg}");
        }

        if let Err(msg) = self.draw_grid(ui) {
            println!("Error!: {msg}");
        }
    }

    pub fn set_palette(&mut self, palette: Palette) {
        self.palette = palette;
    }

    pub fn update(&mut self, ctx: &Context, top_menu_bar_items: Vec<&mut dyn TopMenuBarItem>) {
        TopBottomPanel::top("wrap_app_top_bar")
            .show(ctx, |ui| self.draw_top_menu_bar(ui, top_menu_bar_items));
        CentralPanel::default().show(ctx, |ui| self.draw(ui));
    }
}

include!("tests/canvas_ui_test.rs");
