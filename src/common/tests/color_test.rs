#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn to_color32_test() {
        let color = ODColor::new(1, 2, 3);
        let color_32 = Color32::from_rgb(1, 2, 3);

        assert_eq!(color.to_color32(), color_32);
    }

    #[test]
    fn from_color32_test() {
        let color_32 = Color32::from_rgb(1, 2, 3);
        let color = ODColor::from_color32(color_32);

        assert_eq!(color, ODColor::new(1, 2, 3));
    }

    #[test]
    fn get_set_color_set() {
        let color = ODColor::new(1, 2, 3);
        let mut color_set = ColorSet::new(color);

        let color = color_set.get_color(0).unwrap();
        assert_eq!(color, ODColor::new(1, 2, 3));

        let res = color_set.set_color(1, ODColor::new(2, 4, 6));
        assert!(res.is_ok());

        let color = color_set.get_color(1).unwrap();
        assert_eq!(color, ODColor::new(2, 4, 6));

        let res = color_set.get_color(100);
        assert!(res.is_err());

        let res = color_set.set_color(100, ODColor::new(5, 6, 7));
        assert!(res.is_err());
    }
}
