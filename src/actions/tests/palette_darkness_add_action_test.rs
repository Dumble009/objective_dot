#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::palette::Palette;
    use crate::mock::palette_mock::PaletteMock;

    #[test]
    fn run_undo_test() {
        let palette = Rc::new(RefCell::new(PaletteMock::new()));
        let mut action = PaletteDarknessAddAction::new(palette.clone(), 0);
        assert!(action.run().is_ok());

        let actual = palette.borrow().get_active_darkness_range(0).unwrap();
        assert_eq!(1, actual);

        assert!(action.undo().is_ok());

        let actual = palette.borrow().get_active_darkness_range(0).unwrap();
        assert_eq!(0, actual);
    }
}
