#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::color::ODColor;
    use crate::mock::drawing_mock::DrawingMock;
    use crate::ui_components::draw_modes::rect_fill::RectFill;

    #[test]
    fn rect_fill_basic_test() {
        let mut rect_fill = RectFill::new();
        let mut canvas = vec![vec![0; 5]; 5];
        let canvas_size = (5, 5);
        let mut drawing = DrawingMock::new();

        assert!(drawing
            .get_palette()
            .borrow_mut()
            .add_color(ODColor::new(0, 0, 0))
            .is_ok());
        drawing.get_palette().borrow_mut().select_color(1).unwrap();

        assert!(drawing.get_grid().borrow_mut().set_grid_width(5).is_ok());
        assert!(drawing.get_grid().borrow_mut().set_grid_height(5).is_ok());

        // Draw filled rectangle from (1,1) to (3,3)
        rect_fill
            .on_mouse_down(&mut canvas, &canvas_size, &mut drawing, &(1, 1))
            .unwrap();
        rect_fill
            .on_mouse_drag(&mut canvas, &canvas_size, &mut drawing, &(3, 3))
            .unwrap();
        rect_fill
            .on_mouse_up(&mut canvas, &canvas_size, &mut drawing, &(3, 3))
            .unwrap();

        for y in 1..=3 {
            for x in 1..=3 {
                assert_eq!(canvas[y][x], 1);
                assert_eq!(drawing.get_grid().borrow().get_color(x, y).unwrap(), 1);
            }
        }
    }

    #[test]
    fn rect_fill_reverse_direction_test() {
        let mut rect_fill = RectFill::new();
        let mut canvas = vec![vec![0; 4]; 4];
        let canvas_size = (4, 4);
        let mut drawing = DrawingMock::new();

        drawing
            .get_palette()
            .borrow_mut()
            .add_color(ODColor::new(0, 0, 0))
            .unwrap();
        drawing
            .get_palette()
            .borrow_mut()
            .add_color(ODColor::new(255, 255, 255))
            .unwrap();
        drawing.get_palette().borrow_mut().select_color(2).unwrap();

        drawing.get_grid().borrow_mut().set_grid_width(4).unwrap();
        drawing.get_grid().borrow_mut().set_grid_height(4).unwrap();

        // Draw filled rectangle from (3,3) to (1,1)
        rect_fill
            .on_mouse_down(&mut canvas, &canvas_size, &mut drawing, &(3, 3))
            .unwrap();
        rect_fill
            .on_mouse_drag(&mut canvas, &canvas_size, &mut drawing, &(1, 1))
            .unwrap();
        rect_fill
            .on_mouse_up(&mut canvas, &canvas_size, &mut drawing, &(1, 1))
            .unwrap();

        for y in 1..=3 {
            for x in 1..=3 {
                assert_eq!(canvas[y][x], 2);
                assert_eq!(drawing.get_grid().borrow().get_color(x, y).unwrap(), 2);
            }
        }
    }

    #[test]
    fn rect_one_dot_test() {
        let mut rect_fill = RectFill::new();
        let mut canvas = vec![vec![0; 5]; 5];
        let canvas_size = (5, 5);
        let mut drawing = DrawingMock::new();

        assert!(drawing
            .get_palette()
            .borrow_mut()
            .add_color(ODColor::new(0, 0, 0))
            .is_ok());
        drawing.get_palette().borrow_mut().select_color(1).unwrap();

        assert!(drawing.get_grid().borrow_mut().set_grid_width(5).is_ok());
        assert!(drawing.get_grid().borrow_mut().set_grid_height(5).is_ok());

        rect_fill
            .on_mouse_down(&mut canvas, &canvas_size, &mut drawing, &(2, 2))
            .unwrap();
        rect_fill
            .on_mouse_drag(&mut canvas, &canvas_size, &mut drawing, &(2, 2))
            .unwrap();
        rect_fill
            .on_mouse_up(&mut canvas, &canvas_size, &mut drawing, &(2, 2))
            .unwrap();
        for y in 0..5 {
            for x in 0..5 {
                if x == 2 && y == 2 {
                    assert_eq!(canvas[y][x], 1);
                    assert_eq!(drawing.get_grid().borrow().get_color(x, y).unwrap(), 1);
                } else {
                    assert_eq!(canvas[y][x], 0);
                    assert_eq!(drawing.get_grid().borrow().get_color(x, y).unwrap(), 0);
                }
            }
        }
    }

    #[test]
    fn rect_horizontal_test() {
        let mut rect_fill = RectFill::new();
        let mut canvas = vec![vec![0; 5]; 5];
        let canvas_size = (5, 5);
        let mut drawing = DrawingMock::new();

        assert!(drawing
            .get_palette()
            .borrow_mut()
            .add_color(ODColor::new(0, 0, 0))
            .is_ok());
        drawing.get_palette().borrow_mut().select_color(1).unwrap();

        assert!(drawing.get_grid().borrow_mut().set_grid_width(5).is_ok());
        assert!(drawing.get_grid().borrow_mut().set_grid_height(5).is_ok());

        rect_fill
            .on_mouse_down(&mut canvas, &canvas_size, &mut drawing, &(0, 0))
            .unwrap();
        rect_fill
            .on_mouse_drag(&mut canvas, &canvas_size, &mut drawing, &(2, 0))
            .unwrap();
        rect_fill
            .on_mouse_up(&mut canvas, &canvas_size, &mut drawing, &(4, 0))
            .unwrap();

        for x in 0..5 {
            assert_eq!(canvas[0][x], 1);
            assert_eq!(drawing.get_grid().borrow().get_color(x, 0).unwrap(), 1);
        }
    }

    #[test]
    fn rect_vertical_test() {
        let mut rect_fill = RectFill::new();
        let mut canvas = vec![vec![0; 5]; 5];
        let canvas_size = (5, 5);
        let mut drawing = DrawingMock::new();

        assert!(drawing
            .get_palette()
            .borrow_mut()
            .add_color(ODColor::new(0, 0, 0))
            .is_ok());
        drawing.get_palette().borrow_mut().select_color(1).unwrap();

        assert!(drawing.get_grid().borrow_mut().set_grid_width(5).is_ok());
        assert!(drawing.get_grid().borrow_mut().set_grid_height(5).is_ok());

        rect_fill
            .on_mouse_down(&mut canvas, &canvas_size, &mut drawing, &(0, 0))
            .unwrap();
        rect_fill
            .on_mouse_drag(&mut canvas, &canvas_size, &mut drawing, &(0, 2))
            .unwrap();
        rect_fill
            .on_mouse_up(&mut canvas, &canvas_size, &mut drawing, &(0, 4))
            .unwrap();

        for y in 0..5 {
            assert_eq!(canvas[y][0], 1);
            assert_eq!(drawing.get_grid().borrow().get_color(0, y).unwrap(), 1);
        }
    }

    #[test]
    fn rect_out_canvas_test() {
        let mut rect_fill = RectFill::new();
        let mut canvas = vec![vec![0; 5]; 5];
        let canvas_size = (5, 5);
        let mut drawing = DrawingMock::new();

        assert!(drawing
            .get_palette()
            .borrow_mut()
            .add_color(ODColor::new(0, 0, 0))
            .is_ok());
        drawing.get_palette().borrow_mut().select_color(1).unwrap();

        assert!(drawing.get_grid().borrow_mut().set_grid_width(5).is_ok());
        assert!(drawing.get_grid().borrow_mut().set_grid_height(5).is_ok());

        rect_fill
            .on_mouse_down(&mut canvas, &canvas_size, &mut drawing, &(2, 2))
            .unwrap();
        rect_fill
            .on_mouse_drag(&mut canvas, &canvas_size, &mut drawing, &(8, 0))
            .unwrap();

        for y in 0..=2 {
            for x in 2..5 {
                assert_eq!(canvas[y][x], 1);
            }
        }

        rect_fill
            .on_mouse_drag(&mut canvas, &canvas_size, &mut drawing, &(8, 8))
            .unwrap();

        for y in 2..5 {
            for x in 2..5 {
                assert_eq!(canvas[y][x], 1);
            }
        }

        rect_fill
            .on_mouse_drag(&mut canvas, &canvas_size, &mut drawing, &(0, 8))
            .unwrap();

        for y in 2..5 {
            for x in 0..=2 {
                assert_eq!(canvas[y][x], 1);
            }
        }
    }
}
