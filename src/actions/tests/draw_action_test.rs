#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::grid_mock::GridMock;

    #[test]
    fn run_undo_test() {
        let canvas = Rc::new(RefCell::new(GridMock::new()));
        assert!(canvas.borrow_mut().set_grid_height(3).is_ok());
        assert!(canvas.borrow_mut().set_grid_width(3).is_ok());

        for y in 0..3 {
            for x in 0..3 {
                assert!(canvas.borrow_mut().set_color(x, y, 0).is_ok());
            }
        }

        assert!(canvas.borrow_mut().set_color(0, 0, 1).is_ok());
        assert!(canvas.borrow_mut().set_color(1, 1, 2).is_ok());
        assert!(canvas.borrow_mut().set_color(2, 2, 3).is_ok());

        let mut action = DrawAction::new(canvas.clone(), vec![(0, 0), (1, 1), (2, 2)], 4);
        assert!(action.run().is_ok());

        assert_eq!(canvas.borrow_mut().get_color(0, 0).unwrap(), 4);
        assert_eq!(canvas.borrow_mut().get_color(1, 1).unwrap(), 4);
        assert_eq!(canvas.borrow_mut().get_color(2, 2).unwrap(), 4);

        assert!(action.undo().is_ok());

        assert_eq!(canvas.borrow_mut().get_color(0, 0).unwrap(), 1);
        assert_eq!(canvas.borrow_mut().get_color(1, 1).unwrap(), 2);
        assert_eq!(canvas.borrow_mut().get_color(2, 2).unwrap(), 3);
    }
}
