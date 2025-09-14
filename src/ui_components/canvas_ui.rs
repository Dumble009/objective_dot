use crate::common::drawing::Drawing;
use crate::common::palette::PaletteColorIndex;
use crate::ui_components::draw_modes::draw_mode::DrawMode;
use crate::ui_components::draw_modes::fill::Fill;
use crate::ui_components::draw_modes::line::Line;
use crate::ui_components::draw_modes::pencil::Pencil;
use crate::ui_components::draw_modes::rect_fill::RectFill;
use crate::ui_components::draw_modes::rect_line::RectLine;
use crate::ui_components::grid_renderer::{GridRenderer, SimpleGridRenderer};
use crate::ui_components::input_handler::UserInputHandler;
use eframe::egui::*;

use super::input_handler::InputHandler;
use super::top_menu_bar_item::TopMenuBarItem;

const DEFAULT_SQUARE_SIZE: f32 = 30.0;
const TOP_MENU_BAR_HEIGHT: u32 = 40;
const SCROLL_MAGNI: f32 = 0.1;

pub struct CanvasUi {
    square_root_pos: Pos2,
    square_size: f32,
    current_draw_mode: Box<dyn DrawMode>,
    input_handler: Box<dyn InputHandler>,
    grid_renderer: Box<dyn GridRenderer>,
    draw_modes: Vec<Box<dyn DrawMode>>,
}

impl CanvasUi {
    pub fn new() -> Self {
        CanvasUi {
            square_root_pos: Pos2::new(0.0, 0.0),
            square_size: DEFAULT_SQUARE_SIZE,
            current_draw_mode: Box::new(Pencil::new()),
            input_handler: Box::new(UserInputHandler::new()),
            grid_renderer: Box::new(SimpleGridRenderer::new()),
            draw_modes: vec![
                Box::new(Pencil::new()),
                Box::new(Line::new()),
                Box::new(RectFill::new()),
                Box::new(RectLine::new()),
                Box::new(Fill::new()),
            ],
        }
    }

    fn get_current_mouse_pos_in_idx(&self) -> Result<(i32, i32), String> {
        // cursor_pos はウインドウの左上を (0, 0) とする座標系の値で返ってくる想定
        if let Some(cursor_pos) = self.input_handler.get_mouse_pos() {
            let absolute_cursor_pos = cursor_pos - self.square_root_pos;

            let grid_x = std::cmp::max(0, (absolute_cursor_pos.x / (self.square_size)) as i32);
            let grid_y = std::cmp::max(
                0,
                ((absolute_cursor_pos.y - TOP_MENU_BAR_HEIGHT as f32) / self.square_size) as i32,
            );

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
            .borrow()
            .get_color(grid_x as usize, grid_y as usize)?;
        drawing.get_palette().borrow_mut().select_color(color_idx)?;

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

    fn draw_draw_mode_bar(&mut self, ui: &mut Ui) {
        let mut layout = Layout::left_to_right(Align::Min);
        layout.main_wrap = true;
        ui.with_layout(layout, |ui| {
            for draw_mode in &self.draw_modes {
                if ui.button(draw_mode.get_button_label()).clicked() {
                    self.current_draw_mode = draw_mode.clone();
                }
            }
        });
    }

    fn move_canvas(&mut self) {
        let mov = self.input_handler.get_drag_delta();
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

    fn set_current_drawing_to_canvas(
        &self,
        canvas: &mut [Vec<PaletteColorIndex>],
        drawing: &dyn Drawing,
    ) {
        let grid = drawing.get_grid();
        for (y, row) in canvas.iter_mut().enumerate() {
            for (x, color_idx) in row.iter_mut().enumerate() {
                *color_idx = grid.borrow().get_color(x, y).unwrap_or(0);
            }
        }
    }

    fn draw(&mut self, ui: &mut Ui, ctx: &Context, drawing: &mut dyn Drawing) {
        let (response, _) = ui.allocate_painter(
            ui.available_size_before_wrap(),
            Sense::drag() | Sense::click() | Sense::hover(),
        );
        self.input_handler.update(&response, ctx);

        let grid = drawing.get_grid();
        let mut canvas = vec![
            vec![PaletteColorIndex::default(); grid.borrow().get_grid_width()];
            grid.borrow().get_grid_height()
        ];
        self.set_current_drawing_to_canvas(&mut canvas, drawing);

        if let Ok((mouse_idx_x, mouse_idx_y)) = self.get_current_mouse_pos_in_idx() {
            let canvas_size = (
                grid.borrow().get_grid_width(),
                grid.borrow().get_grid_height(),
            );
            if self.input_handler.is_mouse_down(PointerButton::Primary) {
                if let Err(msg) = self.current_draw_mode.on_mouse_down(
                    &mut canvas,
                    &canvas_size,
                    drawing,
                    &(mouse_idx_x as usize, mouse_idx_y as usize),
                ) {
                    println!("Error!: {msg}");
                }
            }

            if self.input_handler.is_dragged_by(PointerButton::Primary) {
                if let Err(msg) = self.current_draw_mode.on_mouse_drag(
                    &mut canvas,
                    &canvas_size,
                    drawing,
                    &(mouse_idx_x as usize, mouse_idx_y as usize),
                ) {
                    println!("Error!: {msg}");
                }
            }

            if self.input_handler.is_mouse_up(PointerButton::Primary) {
                if let Err(msg) = self.current_draw_mode.on_mouse_up(
                    &mut canvas,
                    &canvas_size,
                    drawing,
                    &(mouse_idx_x as usize, mouse_idx_y as usize),
                ) {
                    println!("Error!: {msg}");
                }
            }

            if self.input_handler.is_clicked_by(PointerButton::Secondary) {
                if let Err(msg) = self.choose_color_from_grid(mouse_idx_x, mouse_idx_y, drawing) {
                    println!("Error!: {msg}");
                }
            }
        }

        if self.input_handler.is_dragged_by(PointerButton::Middle) {
            self.move_canvas();
        }

        if self.input_handler.is_hovered() {
            self.zoom(self.input_handler.get_scroll_delta().y);
        }

        if let Err(msg) = self.grid_renderer.draw(
            ui,
            &canvas,
            drawing.get_palette(),
            self.square_root_pos,
            self.square_size,
            Vec2::new(0.0, TOP_MENU_BAR_HEIGHT as f32),
        ) {
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
        TopBottomPanel::top("draw_mode_top_bar").show(ctx, |ui| self.draw_draw_mode_bar(ui));
        CentralPanel::default().show(ctx, |ui| self.draw(ui, ctx, drawing));
    }
}

include!("tests/canvas_ui_test.rs");
