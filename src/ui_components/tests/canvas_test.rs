#[cfg(test)]
mod test {

    use super::{CanvasGrid, GRID_HEIGHT, GRID_WIDTH};

    #[test]
    fn canvas_grid_get_set_color_test() {
        let mut canvas_grid = CanvasGrid::new();

        let get_res = canvas_grid.get_color(0, 0);
        assert!(get_res.is_ok());
        assert_eq!(get_res.unwrap(), 0);

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
}
