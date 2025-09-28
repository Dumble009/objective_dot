#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::canvas_grid::Grid;
    use crate::mock::grid_mock::GridMock;

    #[test]
    fn run_undo_test() {
        let grid = Rc::new(RefCell::new(GridMock::new()));
        let before_size = (10, 20);
        let res = grid.borrow_mut().set_grid_width(before_size.0);
        assert!(res.is_ok());

        let res = grid.borrow_mut().set_grid_height(before_size.1);
        assert!(res.is_ok());

        let after_size_expected = (30, 40);

        let mut action = GridSizeChangeAction::new(grid.clone(), after_size_expected);
        assert!(action.run().is_ok());
        assert_eq!(after_size_expected.0, grid.borrow().get_grid_width());
        assert_eq!(after_size_expected.1, grid.borrow().get_grid_height());

        assert!(action.undo().is_ok());
        assert_eq!(before_size.0, grid.borrow().get_grid_width());
        assert_eq!(before_size.1, grid.borrow().get_grid_height());
    }
}
