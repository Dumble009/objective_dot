#[cfg(test)]
mod test {
    use super::*;

    use crate::common::color::ODColor;
    use crate::mock::drawing_mock::DrawingMock;

    #[test]
    fn pencil_draw_test() {
        let mut pencil = Pencil::new();
        let mut canvas = vec![vec![0; 10]; 10];
        let canvas_size = (10, 10);
        let mut drawing = DrawingMock::new();

        assert!(drawing
            .get_palette()
            .borrow_mut()
            .add_color(ODColor::new(0, 0, 0))
            .is_ok());
        assert!(drawing.get_palette().borrow_mut().select_color(1).is_ok());

        assert!(drawing.get_grid().borrow_mut().set_grid_width(10).is_ok());
        assert!(drawing.get_grid().borrow_mut().set_grid_height(10).is_ok());
        let mouse_pos = (0, 0);

        assert!(pencil
            .on_mouse_down(&mut canvas, &canvas_size, &mut drawing, &mouse_pos)
            .is_ok());
        assert_eq!(canvas[0][0], 1);
        assert_eq!(drawing.get_grid().borrow().get_color(0, 0).unwrap(), 0);

        let mouse_pos = (0, 4);
        assert!(pencil
            .on_mouse_drag(&mut canvas, &canvas_size, &mut drawing, &mouse_pos)
            .is_ok());
        assert_eq!(canvas[4][0], 1);
        assert_eq!(drawing.get_grid().borrow().get_color(0, 4).unwrap(), 0);

        let mouse_pos = (0, 9);
        let mut action = pencil
            .on_mouse_up(&mut canvas, &canvas_size, &mut drawing, &mouse_pos)
            .unwrap()
            .unwrap();
        assert!(action.run().is_ok());
        assert_eq!(canvas[0][9], 0);
        assert_eq!(drawing.get_grid().borrow().get_color(0, 0).unwrap(), 1);
        assert_eq!(drawing.get_grid().borrow().get_color(0, 4).unwrap(), 1);
        assert_eq!(drawing.get_grid().borrow().get_color(0, 9).unwrap(), 0);
    }

    #[test]
    fn pencil_outside_canvas_test() {
        let mut pencil = Pencil::new();
        let mut canvas = vec![vec![0; 10]; 10];
        let canvas_size = (10, 10);
        let mut drawing = DrawingMock::new();

        assert!(drawing
            .get_palette()
            .borrow_mut()
            .add_color(ODColor::new(0, 0, 0))
            .is_ok());
        assert!(drawing.get_palette().borrow_mut().select_color(1).is_ok());

        assert!(drawing.get_grid().borrow_mut().set_grid_width(10).is_ok());
        assert!(drawing.get_grid().borrow_mut().set_grid_height(10).is_ok());

        // マウス位置がキャンバス外
        let mouse_pos = (11, 11);
        assert!(pencil
            .on_mouse_down(&mut canvas, &canvas_size, &mut drawing, &mouse_pos)
            .is_ok());

        // キャンバス外でマウスダウン→キャンバス内に戻ってくる
        let mouse_pos = (9, 9);
        assert!(pencil
            .on_mouse_drag(&mut canvas, &canvas_size, &mut drawing, &mouse_pos)
            .is_ok());
        assert_eq!(canvas[9][9], 1);
        assert_eq!(drawing.get_grid().borrow().get_color(9, 9).unwrap(), 0);

        // 再びキャンバス外に出る
        let mouse_pos = (12, 13);
        assert!(pencil
            .on_mouse_drag(&mut canvas, &canvas_size, &mut drawing, &mouse_pos)
            .is_ok());

        // そのままキャンバス外でマウスアップ
        let mut action = pencil
            .on_mouse_up(&mut canvas, &canvas_size, &mut drawing, &mouse_pos)
            .unwrap()
            .unwrap();
        assert!(action.run().is_ok());
        assert_eq!(drawing.get_grid().borrow().get_color(9, 9).unwrap(), 1);
    }
}
