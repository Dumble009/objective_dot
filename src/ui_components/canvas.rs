use eframe::egui::*;

const GRID_WIDTH: u32 = 10;
const GRID_HEIGHT: u32 = 10;
const DEFAULT_SQUARE_WIDTH: u32 = 30;
const DEFAULT_SQUARE_HEIGHT: u32 = 30;

#[derive(Default)]
struct CanvasGrid {
    grid: Vec<Vec<u8>>,
}

impl CanvasGrid {
    fn new() -> Self {
        CanvasGrid {
            grid: vec![vec![0; GRID_WIDTH as usize]; GRID_HEIGHT as usize],
        }
    }

    fn coordinate_validation(&self, x: usize, y: usize) -> Result<(), String> {
        if y < self.grid.len() && x < self.grid[y].len() {
            return Ok(());
        } else {
            return Err(format!("args are out of range! x={x}, y={y}"));
        }
    }

    fn set_color(&mut self, x: usize, y: usize, val: u8) -> Result<(), String> {
        self.coordinate_validation(x, y)?;

        self.grid[y][x] = val;

        Ok(())
    }

    fn get_color(&self, x: usize, y: usize) -> Result<u8, String> {
        self.coordinate_validation(x, y)?;
        Ok(self.grid[y][x])
    }
}

#[derive(Default)]
pub struct Canvas {
    grid: CanvasGrid,
}

impl Canvas {
    pub fn new() -> Self {
        Canvas {
            grid: CanvasGrid::new(),
        }
    }

    fn draw(&self, ui: &mut Ui) -> Result<(), String> {
        let mut square_x = 0;
        let mut square_y = 0;

        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                let color = self.grid.get_color(x as usize, y as usize)?;
                let square_rect = Rect::from_min_max(
                    Pos2::new(square_x as f32, square_y as f32),
                    Pos2::new(
                        (square_x + DEFAULT_SQUARE_WIDTH) as f32,
                        (square_y + DEFAULT_SQUARE_HEIGHT) as f32,
                    ),
                );

                let fill_color = if color == 1 {
                    Color32::BLACK
                } else {
                    Color32::WHITE
                };

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
            let grid_y = (cursor_pos.y / (DEFAULT_SQUARE_HEIGHT as f32)) as i32;
            if !(0 <= grid_x
                && grid_x < GRID_WIDTH as i32
                && 0 <= grid_y
                && grid_y < GRID_HEIGHT as i32)
            {
                return Ok(());
            }

            self.grid.set_color(grid_x as usize, grid_y as usize, 1)?;
        }
        Ok(())
    }

    fn update(&mut self, _ctx: &Context, _frame: &mut eframe::Frame, ui: &mut Ui) {
        if let Err(msg) = self.fill_by_cursor(ui) {
            println!("Error!: {msg}");
        }
        if let Err(msg) = self.draw(ui) {
            println!("Error!: {msg}");
        }
    }
}

impl eframe::App for Canvas {
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {}
    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            self.update(ctx, frame, ui);
        });
    }
}

include!("tests/canvas_test.rs");
