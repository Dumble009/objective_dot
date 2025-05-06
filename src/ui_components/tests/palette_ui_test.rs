#[cfg(test)]
mod test {
    use super::*;
    use crate::common::color::ODColor;
    use crate::common::palette::ObjectPalette;

    #[test]
    fn add_color_test() {
        let color = ODColor::new(1, 2, 3);
        let mut palette_ui = PaletteUi::new();
        let mut palette = ObjectPalette::new();

        assert!(palette_ui.add_color(color, &mut palette).is_ok());

        assert_eq!(palette.get_color_count(), 2);

        let added_color = palette.get_color(1).unwrap();
        assert_eq!(added_color, color);
    }
}
