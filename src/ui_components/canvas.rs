use eframe::egui::*;

const GRID_WIDTH: u32 = 10;
const GRID_HEIGHT: u32 = 10;
const DEFAULT_SQUARE_WIDTH: u32 = 30;
const DEFAULT_SQUARE_HEIGHT: u32 = 30;

#[derive(Default)]
struct Grid {
    grid: Vec<Vec<u8>>,
}

impl Grid {
    fn new() -> Self {
        Grid {
            grid: vec![vec![0; GRID_WIDTH as usize]; GRID_HEIGHT as usize],
        }
    }

    fn set_color(&mut self, x: usize, y: usize, val: u8) {
        if y >= self.grid.len() || x >= self.grid.len() {
            return;
        }

        self.grid[y][x] = val;
    }
}

#[derive(Default)]
pub struct Canvas {
    grid: Grid,
}

impl Canvas {
    pub fn new() -> Self {
        Canvas { grid: Grid::new() }
    }

    fn draw_grid(&self, ui: &mut Ui) {
        let mut square_x = 0;
        let mut square_y = 0;

        for row in &self.grid.grid {
            for color in row.iter() {
                let square_rect = Rect::from_min_max(
                    Pos2::new(square_x as f32, square_y as f32),
                    Pos2::new(
                        (square_x + DEFAULT_SQUARE_WIDTH) as f32,
                        (square_y + DEFAULT_SQUARE_HEIGHT) as f32,
                    ),
                );

                let fill_color = if *color == 1 {
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
    }

    fn fill_by_cursor(&mut self, ui: &mut Ui) {
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
                return;
            }

            self.grid.set_color(grid_x as usize, grid_y as usize, 1);
        }
    }
}

impl eframe::App for Canvas {
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {}
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            self.fill_by_cursor(ui);
            self.draw_grid(ui);
        });
    }
}
