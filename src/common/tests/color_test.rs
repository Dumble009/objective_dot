#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn to_color32_test() {
        let color = ODColor::new(1, 2, 3);
        let color_32 = Color32::from_rgb(1, 2, 3);

        assert_eq!(color.to_color32(), color_32);
    }
}
