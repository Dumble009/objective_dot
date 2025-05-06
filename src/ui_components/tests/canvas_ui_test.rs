#[cfg(test)]
mod test {

    use crate::common::{
        canvas_grid::{CanvasGrid, Grid},
        color::ODColor,
        palette::Palette,
    };

    use super::CanvasUi;

    #[test]
    fn canvas_fill_by_cursor_test() {
        let mut grid = CanvasGrid::new();
        let mut canvas_ui = CanvasUi::new();
        let mut palette = Palette::new();
        let color = ODColor::new(1, 2, 3);
        palette.add_color(color).unwrap();
        palette.select_color(1).unwrap();

        canvas_ui
            .fill_by_cursor(0, 0, &mut grid, &mut palette)
            .unwrap();
        let idx = grid.get_color(0, 0).unwrap();
        assert_eq!(idx, 1);
    }

    #[test]
    fn canvas_choose_color_from_grid_test() {
        let grid = CanvasGrid::new();
        let canvas_ui = CanvasUi::new();
        let mut palette = Palette::new();
        let color = ODColor::new(1, 2, 3);
        palette.add_color(color).unwrap();
        palette.select_color(1).unwrap();

        let selected_idx = palette.get_current_selected_idx().unwrap();
        assert_eq!(selected_idx, 1);

        canvas_ui
            .choose_color_from_grid(0, 0, &grid, &mut palette)
            .unwrap();
        let selected_idx = palette.get_current_selected_idx().unwrap();
        assert_eq!(selected_idx, 0);
    }
}
