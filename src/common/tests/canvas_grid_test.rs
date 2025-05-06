#[cfg(test)]
mod test {
    use super::{CanvasGrid, INITIAL_GRID_HEIGHT, INITIAL_GRID_WIDTH};

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
    fn canvas_grid_get_width_height_test() {
        let canvas_grid = CanvasGrid::new();

        let width = canvas_grid.get_grid_width();
        let height = canvas_grid.get_grid_height();

        assert_eq!(width, INITIAL_GRID_WIDTH);
        assert_eq!(height, INITIAL_GRID_HEIGHT);
    }

    #[test]
    fn canvas_grid_out_of_range_test() {
        let mut canvas_grid = CanvasGrid::new();

        let width = canvas_grid.get_grid_width();
        let height = canvas_grid.get_grid_height();

        let get_res = canvas_grid.get_color(width, height);
        assert!(get_res.is_err());

        let set_res = canvas_grid.set_color(width, height, 1);
        assert!(set_res.is_err());
    }
}
