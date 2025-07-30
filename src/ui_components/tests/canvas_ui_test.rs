#[cfg(test)]
mod test {

    use super::CanvasUi;
    use crate::common::{color::ODColor, drawing::Drawing};
    use crate::mock::drawing_mock::DrawingMock;

    #[test]
    fn canvas_choose_color_from_grid_test() {
        let canvas_ui = CanvasUi::new();
        let mut drawing = DrawingMock::new();
        let color = ODColor::new(1, 2, 3);
        let p = drawing.get_palette_mut();
        p.add_color(color).unwrap();
        p.select_color(1).unwrap();

        let selected_idx = p.get_current_selected_idx().unwrap();
        assert_eq!(selected_idx, 1);

        canvas_ui
            .choose_color_from_grid(0, 0, &mut drawing)
            .unwrap();

        let p = drawing.get_palette_mut();
        let selected_idx = p.get_current_selected_idx().unwrap();
        assert_eq!(selected_idx, 0);
    }
}
