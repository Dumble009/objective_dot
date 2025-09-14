#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::palette::Palette;
    use crate::mock::palette_mock::PaletteMock;

    #[test]
    fn run_undo_test() {
        let mut palette = Rc::new(RefCell::new(PaletteMock::new()));
        let bc = palette.borrow().get_color(0).unwrap();
        let ac = ODColor::new(1, 1, 1);
        let mut action = PaletteColorChangeAction::new(palette.clone(), 0, bc, ac);
        action.run();

        let actual = palette.borrow().get_color(0).unwrap();
        assert_eq!(ac, actual);

        action.undo();

        let actual = palette.borrow().get_color(0).unwrap();
        assert_eq!(bc, actual);
    }
}
