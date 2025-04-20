#[cfg(test)]
mod test {
    use super::*;
    use crate::common::color::ODColor;

    #[test]
    fn add_get_color_test() {
        let color = ODColor::new(1, 2, 3);
        let mut palette = Palette::new();

        // 初期状態でも1色は存在している。
        assert_eq!(palette.get_color_count(), 1);

        assert!(palette.add_color(color).is_ok());

        let get_color_res = palette.get_color(1);
        assert!(get_color_res.is_ok());
        assert_eq!(get_color_res.unwrap(), color);

        assert_eq!(palette.get_color_count(), 2);

        assert!(palette.add_color(color).is_ok());

        assert_eq!(palette.get_color_count(), 3);
    }

    #[test]
    fn get_color_invalid_idx_test() {
        let palette = Palette::new();

        let get_color_res = palette.get_color(1);
        assert!(get_color_res.is_err());
    }

    #[test]
    fn select_and_get_selected_color() {
        let mut palette = Palette::new();

        // 初期状態でも1色は存在している。最初はそれが選択されている。
        let get_current_selected_idx_res = palette.get_current_selected_idx();
        assert!(get_current_selected_idx_res.is_ok());
        assert_eq!(get_current_selected_idx_res.unwrap(), 0);

        let color = ODColor::new(1, 2, 3);
        palette.add_color(color).unwrap();

        let select_color_res = palette.select_color(1);
        assert!(select_color_res.is_ok());

        let get_current_selected_idx_res = palette.get_current_selected_idx();
        assert!(get_current_selected_idx_res.is_ok());
        assert_eq!(get_current_selected_idx_res.unwrap(), 1);
    }

    #[test]
    fn select_color_invalid_idx_test() {
        let mut palette = Palette::new();

        let select_color_res = palette.select_color(1);
        assert!(select_color_res.is_err());
    }

    #[test]
    fn get_current_selected_color_test() {
        let mut palette = Palette::new();

        palette.current_selected_idx = 1;

        let get_current_selected_idx_res = palette.get_current_selected_idx();
        assert!(get_current_selected_idx_res.is_err());
    }

    #[test]
    fn change_color_test() {
        let mut palette = Palette::new();

        let color = ODColor::new(1, 2, 3);
        palette.add_color(color).unwrap();

        // 初期状態でも一色は存在するので、新しく追加した色は idx=1 になる。
        let new_color = ODColor::new(2, 3, 4);
        let change_color_res = palette.change_color(1, new_color);
        assert!(change_color_res.is_ok());

        let get_color_res = palette.get_color(1);
        assert!(get_color_res.is_ok());
        let changed_color = get_color_res.unwrap();
        assert_eq!(changed_color, new_color);

        let change_color_res = palette.change_color(2, new_color);
        assert!(change_color_res.is_err());
    }
}
