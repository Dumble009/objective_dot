#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::color::ODColor;
    use crate::mock::drawing_mock::DrawingMock;
    use crate::ui_components::draw_modes::rect_fill::RectFill;

    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn rect_fill_basic_test() {
        let mut rect_fill = RectFill::new();
        let mut canvas = vec![vec![(0, 0).into(); 5]; 5];
        let canvas_size = (5, 5);
        let drawing = Rc::new(RefCell::new(DrawingMock::new()));

        assert!(drawing
            .borrow()
            .get_palette()
            .borrow_mut()
            .add_color(ODColor::new(0, 0, 0))
            .is_ok());
        drawing
            .borrow()
            .get_palette()
            .borrow_mut()
            .select_color((1, 0).into())
            .unwrap();

        assert!(drawing
            .borrow()
            .get_active_layer()
            .borrow_mut()
            .set_grid_width(5)
            .is_ok());
        assert!(drawing
            .borrow()
            .get_active_layer()
            .borrow_mut()
            .set_grid_height(5)
            .is_ok());

        // Draw filled rectangle from (1,1) to (3,3)
        rect_fill
            .on_mouse_down(&mut canvas, &canvas_size, drawing.clone(), &(1, 1))
            .unwrap();
        rect_fill
            .on_mouse_drag(&mut canvas, &canvas_size, drawing.clone(), &(3, 3))
            .unwrap();
        let mut action = rect_fill
            .on_mouse_up(&mut canvas, &canvas_size, drawing.clone(), &(3, 3))
            .unwrap()
            .unwrap();

        assert!(action.run().is_ok());

        for y in 1..=3 {
            for x in 1..=3 {
                assert_eq!(canvas[y][x], (1, 0));
                assert_eq!(
                    drawing
                        .borrow()
                        .get_grid()
                        .unwrap()
                        .get_color(x, y)
                        .unwrap(),
                    (1, 0)
                );
            }
        }
    }

    #[test]
    fn rect_fill_reverse_direction_test() {
        let mut rect_fill = RectFill::new();
        let mut canvas = vec![vec![(0, 0).into(); 4]; 4];
        let canvas_size = (4, 4);
        let drawing = Rc::new(RefCell::new(DrawingMock::new()));

        drawing
            .borrow()
            .get_palette()
            .borrow_mut()
            .add_color(ODColor::new(0, 0, 0))
            .unwrap();
        drawing
            .borrow()
            .get_palette()
            .borrow_mut()
            .add_color(ODColor::new(255, 255, 255))
            .unwrap();
        drawing
            .borrow()
            .get_palette()
            .borrow_mut()
            .select_color((2, 0).into())
            .unwrap();

        drawing
            .borrow()
            .get_active_layer()
            .borrow_mut()
            .set_grid_width(4)
            .unwrap();
        drawing
            .borrow()
            .get_active_layer()
            .borrow_mut()
            .set_grid_height(4)
            .unwrap();

        // Draw filled rectangle from (3,3) to (1,1)
        rect_fill
            .on_mouse_down(&mut canvas, &canvas_size, drawing.clone(), &(3, 3))
            .unwrap();
        rect_fill
            .on_mouse_drag(&mut canvas, &canvas_size, drawing.clone(), &(1, 1))
            .unwrap();
        let mut action = rect_fill
            .on_mouse_up(&mut canvas, &canvas_size, drawing.clone(), &(1, 1))
            .unwrap()
            .unwrap();

        assert!(action.run().is_ok());

        for y in 1..=3 {
            for x in 1..=3 {
                assert_eq!(canvas[y][x], (2, 0));
                assert_eq!(
                    drawing
                        .borrow()
                        .get_grid()
                        .unwrap()
                        .get_color(x, y)
                        .unwrap(),
                    (2, 0)
                );
            }
        }
    }

    #[test]
    fn rect_one_dot_test() {
        let mut rect_fill = RectFill::new();
        let mut canvas = vec![vec![(0, 0).into(); 5]; 5];
        let canvas_size = (5, 5);
        let drawing = Rc::new(RefCell::new(DrawingMock::new()));

        assert!(drawing
            .borrow()
            .get_palette()
            .borrow_mut()
            .add_color(ODColor::new(0, 0, 0))
            .is_ok());
        drawing
            .borrow()
            .get_palette()
            .borrow_mut()
            .select_color((1, 0).into())
            .unwrap();

        assert!(drawing.borrow_mut().set_grid_width(5).is_ok());
        assert!(drawing.borrow_mut().set_grid_height(5).is_ok());

        rect_fill
            .on_mouse_down(&mut canvas, &canvas_size, drawing.clone(), &(2, 2))
            .unwrap();
        rect_fill
            .on_mouse_drag(&mut canvas, &canvas_size, drawing.clone(), &(2, 2))
            .unwrap();
        let mut action = rect_fill
            .on_mouse_up(&mut canvas, &canvas_size, drawing.clone(), &(2, 2))
            .unwrap()
            .unwrap();

        assert!(action.run().is_ok());

        for y in 0..5 {
            for x in 0..5 {
                if x == 2 && y == 2 {
                    assert_eq!(canvas[y][x], (1, 0));
                    assert_eq!(
                        drawing
                            .borrow()
                            .get_grid()
                            .unwrap()
                            .get_color(x, y)
                            .unwrap(),
                        (1, 0)
                    );
                } else {
                    assert_eq!(canvas[y][x], (0, 0));
                    assert_eq!(
                        drawing
                            .borrow()
                            .get_grid()
                            .unwrap()
                            .get_color(x, y)
                            .unwrap(),
                        (0, 0)
                    );
                }
            }
        }
    }

    #[test]
    fn rect_horizontal_test() {
        let mut rect_fill = RectFill::new();
        let mut canvas = vec![vec![(0, 0).into(); 5]; 5];
        let canvas_size = (5, 5);
        let drawing = Rc::new(RefCell::new(DrawingMock::new()));

        assert!(drawing
            .borrow()
            .get_palette()
            .borrow_mut()
            .add_color(ODColor::new(0, 0, 0))
            .is_ok());
        drawing
            .borrow()
            .get_palette()
            .borrow_mut()
            .select_color((1, 0).into())
            .unwrap();

        assert!(drawing.borrow_mut().set_grid_width(5).is_ok());
        assert!(drawing.borrow_mut().set_grid_height(5).is_ok());

        rect_fill
            .on_mouse_down(&mut canvas, &canvas_size, drawing.clone(), &(0, 0))
            .unwrap();
        rect_fill
            .on_mouse_drag(&mut canvas, &canvas_size, drawing.clone(), &(2, 0))
            .unwrap();
        let mut action = rect_fill
            .on_mouse_up(&mut canvas, &canvas_size, drawing.clone(), &(4, 0))
            .unwrap()
            .unwrap();

        assert!(action.run().is_ok());

        for x in 0..5 {
            assert_eq!(canvas[0][x], (1, 0));
            assert_eq!(
                drawing
                    .borrow()
                    .get_grid()
                    .unwrap()
                    .get_color(x, 0)
                    .unwrap(),
                (1, 0)
            );
        }
    }

    #[test]
    fn rect_vertical_test() {
        let mut rect_fill = RectFill::new();
        let mut canvas = vec![vec![(0, 0).into(); 5]; 5];
        let canvas_size = (5, 5);
        let drawing = Rc::new(RefCell::new(DrawingMock::new()));

        assert!(drawing
            .borrow()
            .get_palette()
            .borrow_mut()
            .add_color(ODColor::new(0, 0, 0))
            .is_ok());
        drawing
            .borrow()
            .get_palette()
            .borrow_mut()
            .select_color((1, 0).into())
            .unwrap();

        assert!(drawing.borrow_mut().set_grid_width(5).is_ok());
        assert!(drawing.borrow_mut().set_grid_height(5).is_ok());

        rect_fill
            .on_mouse_down(&mut canvas, &canvas_size, drawing.clone(), &(0, 0))
            .unwrap();
        rect_fill
            .on_mouse_drag(&mut canvas, &canvas_size, drawing.clone(), &(0, 2))
            .unwrap();
        let mut action = rect_fill
            .on_mouse_up(&mut canvas, &canvas_size, drawing.clone(), &(0, 4))
            .unwrap()
            .unwrap();

        assert!(action.run().is_ok());

        for y in 0..5 {
            assert_eq!(canvas[y][0], (1, 0));
            assert_eq!(
                drawing
                    .borrow()
                    .get_grid()
                    .unwrap()
                    .get_color(0, y)
                    .unwrap(),
                (1, 0)
            );
        }
    }

    #[test]
    fn rect_out_canvas_test() {
        let mut rect_fill = RectFill::new();
        let mut canvas = vec![vec![(0, 0).into(); 5]; 5];
        let canvas_size = (5, 5);
        let drawing = Rc::new(RefCell::new(DrawingMock::new()));

        assert!(drawing
            .borrow()
            .get_palette()
            .borrow_mut()
            .add_color(ODColor::new(0, 0, 0))
            .is_ok());
        drawing
            .borrow()
            .get_palette()
            .borrow_mut()
            .select_color((1, 0).into())
            .unwrap();

        assert!(drawing.borrow_mut().set_grid_width(5).is_ok());
        assert!(drawing.borrow_mut().set_grid_height(5).is_ok());

        rect_fill
            .on_mouse_down(&mut canvas, &canvas_size, drawing.clone(), &(2, 2))
            .unwrap();
        rect_fill
            .on_mouse_drag(&mut canvas, &canvas_size, drawing.clone(), &(8, 0))
            .unwrap();

        for y in 0..=2 {
            for x in 2..5 {
                assert_eq!(canvas[y][x], (1, 0));
            }
        }

        rect_fill
            .on_mouse_drag(&mut canvas, &canvas_size, drawing.clone(), &(8, 8))
            .unwrap();

        for y in 2..5 {
            for x in 2..5 {
                assert_eq!(canvas[y][x], (1, 0));
            }
        }

        rect_fill
            .on_mouse_drag(&mut canvas, &canvas_size, drawing.clone(), &(0, 8))
            .unwrap();

        for y in 2..5 {
            for x in 0..=2 {
                assert_eq!(canvas[y][x], (1, 0));
            }
        }
    }
}
