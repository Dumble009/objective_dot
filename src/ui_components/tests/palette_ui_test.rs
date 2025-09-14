#[cfg(test)]
mod test {
    use super::*;
    use crate::common::color::ODColor;
    use crate::mock::palette_mock::PaletteMock;

    #[test]
    fn add_color_test() {
        let color = ODColor::new(1, 2, 3);
        let mut palette_ui = PaletteUi::new();
        let palette = Rc::new(RefCell::new(PaletteMock::new()));

        assert!(palette_ui.add_color(color, palette.clone()).is_ok());

        assert_eq!(palette.borrow().get_color_count(), 2);

        let added_color = palette.borrow().get_color(1).unwrap();
        assert_eq!(added_color, color);
    }
}
