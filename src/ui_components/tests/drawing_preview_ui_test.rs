#[cfg(test)]
mod test {

    use eframe::egui::{Pos2, Rect, Vec2};
    use super::DrawingPreviewUi;
    use assert_float_eq::assert_float_relative_eq;

    #[test]
    fn canvas_fill_by_cursor_square_test() {
        let drawing_preview_ui = DrawingPreviewUi::new();

        let width = 200.0;
        let height = 200.0;
        let grid_w = 10;
        let grid_h = 10;
        let window_size = Vec2::new(width, height);
        let (pos, cell_size) = drawing_preview_ui.calc_drawing_element(
            Rect::from_min_size(Pos2::new(0.0, 0.0), window_size),
            grid_w,
            grid_h
        );

        assert_float_relative_eq!(pos.x, 0.0);
        assert_float_relative_eq!(pos.y, 0.0);

        assert_eq!(cell_size, 20);
    }

    #[test]
    fn canvas_fill_by_cursor_height_long_test() {
        let drawing_preview_ui = DrawingPreviewUi::new();

        let width = 200.0;
        let height = 300.0;
        let grid_w = 10;
        let grid_h = 10;
        let window_size = Vec2::new(width, height);
        let (pos, cell_size) = drawing_preview_ui.calc_drawing_element(
            Rect::from_min_size(Pos2::new(0.0, 0.0), window_size),
            grid_w,
            grid_h
        );

        assert_float_relative_eq!(pos.x, 0.0);
        assert_float_relative_eq!(pos.y, (height - width) / 2.0);

        assert_eq!(cell_size, 20);
    }

    #[test]
    fn canvas_fill_by_cursor_width_long_test() {
        let drawing_preview_ui = DrawingPreviewUi::new();

        let width = 300.0;
        let height = 200.0;
        let grid_w = 10;
        let grid_h = 10;
        let window_size = Vec2::new(width, height);
        let (pos, cell_size) = drawing_preview_ui.calc_drawing_element(
            Rect::from_min_size(Pos2::new(0.0, 0.0), window_size),
            grid_w,
            grid_h
        );

        assert_float_relative_eq!(pos.x, (width - height) / 2.0);
        assert_float_relative_eq!(pos.y, 0.0);

        assert_eq!(cell_size, 20);
    }
}
