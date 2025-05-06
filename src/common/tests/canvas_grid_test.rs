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

    #[test]
    fn canvas_grid_set_width_height_test() {
        let mut canvas_grid = CanvasGrid::new();

        let new_width = INITIAL_GRID_WIDTH + 1;
        let new_height = INITIAL_GRID_HEIGHT + 10;

        let res = canvas_grid.set_grid_width(new_width);
        assert!(res.is_ok());
        let res = canvas_grid.set_grid_height(new_height);
        assert!(res.is_ok());

        let width = canvas_grid.get_grid_width();
        let height = canvas_grid.get_grid_height();

        assert_eq!(width, new_width);
        assert_eq!(height, new_height);

        // out of range にならずにアクセスできる範囲も正常に広がっているか?
        let get_res = canvas_grid.get_color(INITIAL_GRID_WIDTH, INITIAL_GRID_HEIGHT);
        assert!(get_res.is_ok());

        let set_res = canvas_grid.set_color(INITIAL_GRID_WIDTH, INITIAL_GRID_HEIGHT, 1);
        assert!(set_res.is_ok());

        let res = canvas_grid.set_grid_width(INITIAL_GRID_WIDTH);
        assert!(res.is_ok());
        let res = canvas_grid.set_grid_height(INITIAL_GRID_HEIGHT);
        assert!(res.is_ok());

        let width = canvas_grid.get_grid_width();
        let height = canvas_grid.get_grid_height();

        assert_eq!(width, INITIAL_GRID_WIDTH);
        assert_eq!(height, INITIAL_GRID_HEIGHT);

        // 縮小したら out of range になる範囲もそれに合わせて変わっているか?
        let get_res = canvas_grid.get_color(INITIAL_GRID_WIDTH, INITIAL_GRID_HEIGHT);
        assert!(get_res.is_err());

        let set_res = canvas_grid.set_color(INITIAL_GRID_WIDTH, INITIAL_GRID_HEIGHT, 1);
        assert!(set_res.is_err());

        // 現在と同じ値にも設定できる
        let res = canvas_grid.set_grid_width(INITIAL_GRID_WIDTH);
        assert!(res.is_ok());
        let res = canvas_grid.set_grid_height(INITIAL_GRID_HEIGHT);
        assert!(res.is_ok());

        // 0には設定できない
        let res = canvas_grid.set_grid_width(0);
        assert!(res.is_err());
        let res = canvas_grid.set_grid_height(0);
        assert!(res.is_err());

        // 0に設定しようとしても値は変わらない
        let width = canvas_grid.get_grid_width();
        let height = canvas_grid.get_grid_height();

        assert_eq!(width, INITIAL_GRID_WIDTH);
        assert_eq!(height, INITIAL_GRID_HEIGHT);
    }
}
