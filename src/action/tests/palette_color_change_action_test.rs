#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::palette::Palette;
    use crate::mock::palette_mock::PaletteMock;

    #[test]
    fn run_undo_test() {
        let palette = Rc::new(RefCell::new(PaletteMock::new()));
        let bc = palette.borrow().get_color(0).unwrap();
        let ac = ODColor::new(1, 1, 1);
        let mut action = PaletteColorChangeAction::new(palette.clone(), 0, bc, ac);
        assert!(action.run().is_ok());

        let actual = palette.borrow().get_color(0).unwrap();
        assert_eq!(ac, actual);

        assert!(action.undo().is_ok());

        let actual = palette.borrow().get_color(0).unwrap();
        assert_eq!(bc, actual);
    }
}
