#[cfg(test)]
mod test {
    use super::*;
    use crate::common::palette::Palette;
    use crate::mock::palette_mock::PaletteMock;

    #[test]
    fn run_undo_test() {
        let palette = Rc::new(RefCell::new(PaletteMock::new()));
        let color = ODColor::new(1, 1, 1);
        let default_color = palette.borrow().get_color(0).unwrap();

        let mut action = PaletteColorAddAction::new(palette.clone(), color);
        assert!(action.run().is_ok());

        assert_eq!(2, palette.borrow().get_color_count());

        let new_color = palette.borrow().get_color(1);
        assert!(new_color.is_ok());
        assert_eq!(color, new_color.unwrap());

        assert!(action.undo().is_ok());

        assert_eq!(1, palette.borrow().get_color_count());
        assert_eq!(default_color, palette.borrow().get_color(0).unwrap());
    }
}
