#[cfg(test)]
mod test {
    use super::*;
    use crate::common::color::ODColor;

    #[test]
    fn add_color_test() {
        let color = ODColor::new(1, 2, 3);
        let mut palette_ui = PaletteUi::new();

        assert!(palette_ui.add_color(color).is_ok());
    }

    #[test]
    fn clone_palette_test() {
        let mut palette_ui = PaletteUi::new();

        palette_ui.add_color(ODColor::default()).unwrap();

        let palette = palette_ui.clone_palette();

        assert_eq!(palette.get_color_count(), 2);
    }
}
