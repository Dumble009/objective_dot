#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::color::ODColor;
    use crate::common::palette::Palette;
    use crate::mock::palette_mock::PaletteMock;

    #[test]
    fn run_undo_test() {
        let palette = Rc::new(RefCell::new(PaletteMock::new()));

        let ret = palette.borrow_mut().add_color(ODColor::new(1, 1, 1));
        assert!(ret.is_ok());

        let mut action =
            PaletteSelectColorAction::new(palette.clone(), (0, 0).into(), (1, 0).into());
        assert!(action.run().is_ok());

        let actual = palette.borrow().get_current_selected_idx().unwrap();
        assert_eq!((1, 0), actual);

        assert!(action.undo().is_ok());

        let actual = palette.borrow().get_current_selected_idx().unwrap();
        assert_eq!((0, 0), actual);
    }
}
