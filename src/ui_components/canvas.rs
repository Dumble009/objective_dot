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

        let grid_stroke = Stroke::new(1.0, Color32::BLACK);
        for row in &self.grid.grid {
            for _square in row {
                let square_rect = Rect::from_min_max(
                    Pos2::new(square_x as f32, square_y as f32),
                    Pos2::new(
                        (square_x + DEFAULT_SQUARE_WIDTH) as f32,
                        (square_y + DEFAULT_SQUARE_HEIGHT) as f32,
                    ),
                );

                ui.painter().rect(
                    square_rect,
                    0,
                    Color32::WHITE,
                    grid_stroke,
                    StrokeKind::Middle,
                );

                square_x += DEFAULT_SQUARE_WIDTH;
            }
            square_x = 0;
            square_y += DEFAULT_SQUARE_HEIGHT;
        }
    }
}

impl eframe::App for Canvas {
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {}
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            self.draw_grid(ui);
        });
    }
}
