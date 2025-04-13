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
            ColorPickResult::AddNewColor(ODColor::from_color32(Color32::RED))
        );

        color_picker_ui.show("", ColorPickMode::AddNewColor);
        let result = color_picker_ui.get_color();
        assert_eq!(result, ColorPickResult::Waiting);
    }

    #[test]
    fn show_hide_test() {
        let mut color_picker_ui = ColorPickerUi::new();
        let expect_title = "title";
        color_picker_ui.show(expect_title, ColorPickMode::AddNewColor);
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

    #[test]
    fn change_existing_color_test() {
        let mut color_picker_ui = ColorPickerUi::new();
        let idx = 1;
        color_picker_ui.show("", ColorPickMode::ChangeColor(idx));

        color_picker_ui.set_color(Color32::RED);

        let result = color_picker_ui.get_color();
        assert_eq!(
            result,
            ColorPickResult::ChangeColor(idx, ODColor::from_color32(Color32::RED))
        );
    }

    #[test]
    fn multiple_show_test() {
        let mut color_picker_ui = ColorPickerUi::new();
        let title1 = "Title1";
        color_picker_ui.show(title1, ColorPickMode::AddNewColor);

        let title2 = "Title2";
        color_picker_ui.show(title2, ColorPickMode::AddNewColor);
        assert_eq!(color_picker_ui.window_title, title1);

        color_picker_ui.show(title2, ColorPickMode::ChangeColor(1));
        color_picker_ui.set_color(Color32::RED);
        let result = color_picker_ui.get_color();
        assert_eq!(
            result,
            ColorPickResult::AddNewColor(ODColor::from_color32(Color32::RED))
        );
    }
}
