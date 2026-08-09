#[cfg(test)]
mod test {

    use super::CanvasUi;
    use crate::common::{color::ODColor, drawing::Drawing};
    use crate::mock::drawing_mock::DrawingMock;

    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn canvas_choose_color_from_grid_test() {
        let canvas_ui = CanvasUi::new();
        let drawing = Rc::new(RefCell::new(DrawingMock::new()));
        let color = ODColor::new(1, 2, 3);
        let p = drawing.borrow().get_palette();
        p.borrow_mut().add_color(color).unwrap();
        p.borrow_mut().select_color((1, 0).into()).unwrap();

        let selected_idx = p.borrow().get_current_selected_idx().unwrap();
        assert_eq!(selected_idx, (1, 0));

        canvas_ui.choose_color_from_grid(0, 0, drawing).unwrap();

        let selected_idx = p.borrow().get_current_selected_idx().unwrap();
        assert_eq!(selected_idx, (0, 0));
    }
}
