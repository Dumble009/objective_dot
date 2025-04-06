#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn pick_color_test() {
        let mut color_picker_ui = ColorPickerUi::new();
        let result = color_picker_ui.get_color();
        assert_eq!(result, ColorPickResult::Waiting);

        color_picker_ui.set_color(Color32::RED);

        let result = color_picker_ui.get_color();
        assert_eq!(
            result,
            ColorPickResult::Picked(ODColor::from_color32(Color32::RED))
        );

        color_picker_ui.show("");
        let result = color_picker_ui.get_color();
        assert_eq!(result, ColorPickResult::Waiting);
    }

    #[test]
    fn show_hide_test() {
        let mut color_picker_ui = ColorPickerUi::new();
        let expect_title = "title";
        color_picker_ui.show(expect_title);
        assert_eq!(color_picker_ui.is_showing(), true);
        assert_eq!(color_picker_ui.window_title, expect_title);

        color_picker_ui.hide();
        assert_eq!(color_picker_ui.is_showing(), false);
    }

    #[test]
    fn cancel_test() {
        let mut color_picker_ui = ColorPickerUi::new();
        let result = color_picker_ui.get_color();
        assert_eq!(result, ColorPickResult::Waiting);

        color_picker_ui.cancel();
        let result = color_picker_ui.get_color();
        assert_eq!(result, ColorPickResult::Canceled);
    }
}
