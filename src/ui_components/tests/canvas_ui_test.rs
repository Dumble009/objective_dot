#[cfg(test)]
mod test {

    use crate::common::{color::ODColor, palette::Palette};

    use super::{CanvasGrid, CanvasUi, GRID_HEIGHT, GRID_WIDTH};

    #[test]
    fn canvas_grid_get_set_color_test() {
        let mut canvas_grid = CanvasGrid::new();

        let get_res = canvas_grid.get_color(0, 0);
        assert!(get_res.is_ok());

        assert!(canvas_grid.set_color(0, 0, 1).is_ok());

        let get_res = canvas_grid.get_color(0, 0);
        assert!(get_res.is_ok());
        assert_eq!(get_res.unwrap(), 1);
    }

    #[test]
    fn canvas_grid_out_of_range_test() {
        let mut canvas_grid = CanvasGrid::new();

        let get_res = canvas_grid.get_color(GRID_WIDTH as usize, GRID_HEIGHT as usize);
        assert!(get_res.is_err());

        let set_res = canvas_grid.set_color(GRID_WIDTH as usize, GRID_HEIGHT as usize, 1);
        assert!(set_res.is_err());
    }

    #[test]
    fn canvas_fill_by_cursor_test() {
        let mut canvas_ui = CanvasUi::new();
        let mut palette = Palette::new();
        let color = ODColor::new(1, 2, 3);
        palette.add_color(color).unwrap();
        palette.select_color(1).unwrap();

        canvas_ui.fill_by_cursor(0, 0, &mut palette).unwrap();
        let idx = canvas_ui.grid.get_color(0, 0).unwrap();
        assert_eq!(idx, 1);
    }

    #[test]
    fn canvas_choose_color_from_grid_test() {
        let canvas_ui = CanvasUi::new();
        let mut palette = Palette::new();
        let color = ODColor::new(1, 2, 3);
        palette.add_color(color).unwrap();
        palette.select_color(1).unwrap();

        canvas_ui
            .choose_color_from_grid(0, 0, &mut palette)
            .unwrap();
        let selected_idx = palette.get_current_active_idx().unwrap();
        assert_eq!(selected_idx, 0);
    }
}
