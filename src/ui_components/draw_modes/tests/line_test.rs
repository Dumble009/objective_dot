#[cfg(test)]
mod test {
    use super::*;

    use crate::common::color::ODColor;
    use crate::mock::drawing_mock::DrawingMock;

    const CANVAS_EDGE_SIZE: usize = 10;

    #[test]
    fn line_draw_test1() {
        let mut line = Line::new();
        let mut canvas = vec![vec![0; CANVAS_EDGE_SIZE]; CANVAS_EDGE_SIZE];
        let canvas_size = (CANVAS_EDGE_SIZE, CANVAS_EDGE_SIZE);
        let mut drawing = DrawingMock::new();
        let palette = drawing.get_palette();
        assert!(palette
            .borrow_mut()
            .add_color(ODColor::new(0, 0, 0))
            .is_ok());
        assert!(palette.borrow_mut().select_color(1).is_ok());
        let grid = drawing.get_grid();
        assert!(grid.borrow_mut().set_grid_width(CANVAS_EDGE_SIZE).is_ok());
        assert!(grid.borrow_mut().set_grid_height(CANVAS_EDGE_SIZE).is_ok());

        // Start drawing a line
        let mouse_pos = (0, 0);
        assert!(line
            .on_mouse_down(&mut canvas, &canvas_size, &mut drawing, &mouse_pos)
            .is_ok());
        // 表示上は塗られるが、この時点ではまだ本当のグリッドには反映されない
        assert_eq!(canvas[0][0], 1);
        assert_eq!(drawing.get_grid().borrow().get_color(0, 0).unwrap(), 0);

        // Dragging the mouse to draw the line
        let mouse_pos = (5, 5);
        assert!(line
            .on_mouse_drag(&mut canvas, &canvas_size, &mut drawing, &mouse_pos)
            .is_ok());
        for i in 0..6 {
            assert_eq!(canvas[i][i], 1);
            assert_eq!(drawing.get_grid().borrow().get_color(i, i).unwrap(), 0);
        }

        // Finish drawing the line
        let mouse_pos = (9, 9);
        assert!(line
            .on_mouse_up(&mut canvas, &canvas_size, &mut drawing, &mouse_pos)
            .is_ok());
        // マウスを離した時点で本当のグリッドに反映される
        for i in 0..10 {
            assert_eq!(canvas[i][i], 1);
            assert_eq!(drawing.get_grid().borrow().get_color(i, i).unwrap(), 1);
        }
    }

    #[test]
    fn line_draw_test2() {
        let mut line = Line::new();
        let mut canvas = vec![vec![0; CANVAS_EDGE_SIZE]; CANVAS_EDGE_SIZE];
        let canvas_size = (CANVAS_EDGE_SIZE, CANVAS_EDGE_SIZE);
        let mut drawing = DrawingMock::new();
        let palette = drawing.get_palette();
        assert!(palette
            .borrow_mut()
            .add_color(ODColor::new(0, 0, 0))
            .is_ok());
        assert!(palette.borrow_mut().select_color(1).is_ok());
        let grid = drawing.get_grid();
        assert!(grid.borrow_mut().set_grid_width(CANVAS_EDGE_SIZE).is_ok());
        assert!(grid.borrow_mut().set_grid_height(CANVAS_EDGE_SIZE).is_ok());

        // Start drawing a line
        let mouse_pos_start = (9, 9);
        assert!(line
            .on_mouse_down(&mut canvas, &canvas_size, &mut drawing, &mouse_pos_start)
            .is_ok());
        // 表示上は塗られるが、この時点ではまだ本当のグリッドには反映されない
        assert_eq!(canvas[9][9], 1);
        assert_eq!(drawing.get_grid().borrow().get_color(9, 9).unwrap(), 0);

        // Dragging the mouse to draw the line
        let mouse_pos_drag = (5, 5);
        assert!(line
            .on_mouse_drag(&mut canvas, &canvas_size, &mut drawing, &mouse_pos_drag)
            .is_ok());
        for i in 0..5 {
            assert_eq!(canvas[9 - i][9 - i], 1);
            assert_eq!(
                drawing.get_grid().borrow().get_color(9 - i, 9 - i).unwrap(),
                0
            );
        }

        // Finish drawing the line
        let mouse_pos_end = (0, 0);
        assert!(line
            .on_mouse_up(&mut canvas, &canvas_size, &mut drawing, &mouse_pos_end)
            .is_ok());
        // マウスを離した時点で本当のグリッドに反映される
        for i in 0..10 {
            assert_eq!(canvas[i][i], 1);
            assert_eq!(drawing.get_grid().borrow().get_color(i, i).unwrap(), 1);
        }
    }

    #[test]
    fn line_draw_test3() {
        let mut line = Line::new();
        let mut canvas = vec![vec![0; CANVAS_EDGE_SIZE]; CANVAS_EDGE_SIZE];
        let canvas_size = (CANVAS_EDGE_SIZE, CANVAS_EDGE_SIZE);
        let mut drawing = DrawingMock::new();
        let palette = drawing.get_palette();
        assert!(palette
            .borrow_mut()
            .add_color(ODColor::new(0, 0, 0))
            .is_ok());
        assert!(palette.borrow_mut().select_color(1).is_ok());
        let grid = drawing.get_grid();
        assert!(grid.borrow_mut().set_grid_width(CANVAS_EDGE_SIZE).is_ok());
        assert!(grid.borrow_mut().set_grid_height(CANVAS_EDGE_SIZE).is_ok());

        // Start drawing a line
        let mouse_pos = (9, 0);
        assert!(line
            .on_mouse_down(&mut canvas, &canvas_size, &mut drawing, &mouse_pos)
            .is_ok());
        // 表示上は塗られるが、この時点ではまだ本当のグリッドには反映されない
        assert_eq!(canvas[0][9], 1);
        assert_eq!(drawing.get_grid().borrow().get_color(9, 0).unwrap(), 0);

        // Dragging the mouse to draw the line
        let mouse_pos = (5, 4);
        assert!(line
            .on_mouse_drag(&mut canvas, &canvas_size, &mut drawing, &mouse_pos)
            .is_ok());
        for i in 0..5 {
            println!("test {i}");
            assert_eq!(canvas[i][9 - i], 1);
            assert_eq!(drawing.get_grid().borrow().get_color(9 - i, i).unwrap(), 0);
        }

        // Finish drawing the line
        let mouse_pos = (0, 9);
        assert!(line
            .on_mouse_up(&mut canvas, &canvas_size, &mut drawing, &mouse_pos)
            .is_ok());
        // マウスを離した時点で本当のグリッドに反映される
        for i in 0..10 {
            assert_eq!(canvas[i][9 - i], 1);
            assert_eq!(
                drawing.get_grid().borrow_mut().get_color(9 - i, i).unwrap(),
                1
            );
        }
    }

    #[test]
    fn line_draw_test4() {
        let mut line = Line::new();
        let mut canvas = vec![vec![0; CANVAS_EDGE_SIZE]; CANVAS_EDGE_SIZE];
        let canvas_size = (CANVAS_EDGE_SIZE, CANVAS_EDGE_SIZE);
        let mut drawing = DrawingMock::new();

        assert!(drawing
            .get_palette()
            .borrow_mut()
            .add_color(ODColor::new(0, 0, 0))
            .is_ok());
        assert!(drawing.get_palette().borrow_mut().select_color(1).is_ok());

        assert!(drawing
            .get_grid()
            .borrow_mut()
            .set_grid_width(CANVAS_EDGE_SIZE)
            .is_ok());
        assert!(drawing
            .get_grid()
            .borrow_mut()
            .set_grid_height(CANVAS_EDGE_SIZE)
            .is_ok());

        // Start drawing a line
        let mouse_pos_start = (0, 9);
        assert!(line
            .on_mouse_down(&mut canvas, &canvas_size, &mut drawing, &mouse_pos_start)
            .is_ok());
        // 表示上は塗られるが、この時点ではまだ本当のグリッドには反映されない
        assert_eq!(canvas[9][0], 1);
        assert_eq!(drawing.get_grid().borrow().get_color(0, 9).unwrap(), 0);

        // Dragging the mouse to draw the line
        let mouse_pos_drag = (4, 5);
        assert!(line
            .on_mouse_drag(&mut canvas, &canvas_size, &mut drawing, &mouse_pos_drag)
            .is_ok());
        for i in 0..5 {
            assert_eq!(canvas[9 - i][i], 1);
            assert_eq!(drawing.get_grid().borrow().get_color(i, 9 - i).unwrap(), 0);
        }

        // Finish drawing the line
        let mouse_pos_end = (9, 0);
        assert!(line
            .on_mouse_up(&mut canvas, &canvas_size, &mut drawing, &mouse_pos_end)
            .is_ok());
        // マウスを離した時点で本当のグリッドに反映される
        for i in 0..10 {
            assert_eq!(canvas[9 - i][i], 1);
            assert_eq!(drawing.get_grid().borrow().get_color(i, 9 - i).unwrap(), 1);
        }
    }

    #[test]
    fn line_outside_canvas_test() {
        let mut line = Line::new();
        let mut canvas = vec![vec![0; CANVAS_EDGE_SIZE]; CANVAS_EDGE_SIZE];
        let canvas_size = (CANVAS_EDGE_SIZE, CANVAS_EDGE_SIZE);
        let mut drawing = DrawingMock::new();

        assert!(drawing
            .get_palette()
            .borrow_mut()
            .add_color(ODColor::new(0, 0, 0))
            .is_ok());
        assert!(drawing.get_palette().borrow_mut().select_color(1).is_ok());

        assert!(drawing
            .get_grid()
            .borrow_mut()
            .set_grid_width(CANVAS_EDGE_SIZE)
            .is_ok());
        assert!(drawing
            .get_grid()
            .borrow_mut()
            .set_grid_height(CANVAS_EDGE_SIZE)
            .is_ok());

        let mouse_pos = (7, 10);
        assert!(line
            .on_mouse_down(&mut canvas, &canvas_size, &mut drawing, &mouse_pos)
            .is_ok());

        let mouse_pos = (10, 7);
        assert!(line
            .on_mouse_drag(&mut canvas, &canvas_size, &mut drawing, &mouse_pos)
            .is_ok());

        for i in 0..2 {
            assert_eq!(canvas[9 - i][8 + i], 1);
            assert_eq!(
                drawing.get_grid().borrow().get_color(8 + i, 9 - i).unwrap(),
                0
            );
        }

        assert!(line
            .on_mouse_up(&mut canvas, &canvas_size, &mut drawing, &mouse_pos)
            .is_ok());
        for i in 0..2 {
            assert_eq!(canvas[9 - i][8 + i], 1);
            assert_eq!(
                drawing.get_grid().borrow().get_color(8 + i, 9 - i).unwrap(),
                1
            );
        }
    }

    #[test]
    fn line_not_1x1_draw_test() {
        let mut line = Line::new();
        let mut canvas = vec![vec![0; CANVAS_EDGE_SIZE]; CANVAS_EDGE_SIZE];
        let canvas_size = (CANVAS_EDGE_SIZE, CANVAS_EDGE_SIZE);
        let mut drawing = DrawingMock::new();

        assert!(drawing
            .get_palette()
            .borrow_mut()
            .add_color(ODColor::new(0, 0, 0))
            .is_ok());
        assert!(drawing.get_palette().borrow_mut().select_color(1).is_ok());

        assert!(drawing
            .get_grid()
            .borrow_mut()
            .set_grid_width(CANVAS_EDGE_SIZE)
            .is_ok());
        assert!(drawing
            .get_grid()
            .borrow_mut()
            .set_grid_height(CANVAS_EDGE_SIZE)
            .is_ok());

        let mouse_pos = (4, 4);
        assert!(line
            .on_mouse_down(&mut canvas, &canvas_size, &mut drawing, &mouse_pos)
            .is_ok());
        assert_eq!(canvas[4][4], 1);
        assert_eq!(drawing.get_grid().borrow().get_color(4, 4).unwrap(), 0);

        let mouse_pos = (0, 1);
        assert!(line
            .on_mouse_drag(&mut canvas, &canvas_size, &mut drawing, &mouse_pos)
            .is_ok());
        assert_eq!(canvas[4][4], 1);
        assert_eq!(canvas[3][3], 1);
        assert_eq!(canvas[3][2], 1);
        assert_eq!(canvas[2][1], 1);
        assert_eq!(canvas[1][0], 1);

        let mouse_pos = (1, 0);
        assert!(line
            .on_mouse_drag(&mut canvas, &canvas_size, &mut drawing, &mouse_pos)
            .is_ok());
        assert_eq!(canvas[4][4], 1);
        assert_eq!(canvas[3][3], 1);
        assert_eq!(canvas[2][3], 1);
        assert_eq!(canvas[1][2], 1);
        assert_eq!(canvas[0][1], 1);

        let mouse_pos = (7, 0);
        assert!(line
            .on_mouse_drag(&mut canvas, &canvas_size, &mut drawing, &mouse_pos)
            .is_ok());
        assert_eq!(canvas[4][4], 1);
        assert_eq!(canvas[3][5], 1);
        assert_eq!(canvas[2][5], 1);
        assert_eq!(canvas[1][6], 1);
        assert_eq!(canvas[0][7], 1);

        let mouse_pos = (8, 1);
        assert!(line
            .on_mouse_drag(&mut canvas, &canvas_size, &mut drawing, &mouse_pos)
            .is_ok());
        assert_eq!(canvas[4][4], 1);
        assert_eq!(canvas[3][5], 1);
        assert_eq!(canvas[2][6], 1);
        assert_eq!(canvas[2][7], 1);
        assert_eq!(canvas[1][8], 1);

        let mouse_pos = (8, 7);
        assert!(line
            .on_mouse_drag(&mut canvas, &canvas_size, &mut drawing, &mouse_pos)
            .is_ok());
        assert_eq!(canvas[4][4], 1);
        assert_eq!(canvas[5][5], 1);
        assert_eq!(canvas[6][6], 1);
        assert_eq!(canvas[6][7], 1);
        assert_eq!(canvas[7][8], 1);

        let mouse_pos = (7, 8);
        assert!(line
            .on_mouse_drag(&mut canvas, &canvas_size, &mut drawing, &mouse_pos)
            .is_ok());
        assert_eq!(canvas[4][4], 1);
        assert_eq!(canvas[5][5], 1);
        assert_eq!(canvas[6][6], 1);
        assert_eq!(canvas[7][6], 1);
        assert_eq!(canvas[8][7], 1);

        let mouse_pos = (1, 8);
        assert!(line
            .on_mouse_drag(&mut canvas, &canvas_size, &mut drawing, &mouse_pos)
            .is_ok());
        assert_eq!(canvas[4][4], 1);
        assert_eq!(canvas[5][3], 1);
        assert_eq!(canvas[6][2], 1);
        assert_eq!(canvas[7][2], 1);
        assert_eq!(canvas[8][1], 1);

        let mouse_pos = (0, 7);
        assert!(line
            .on_mouse_drag(&mut canvas, &canvas_size, &mut drawing, &mouse_pos)
            .is_ok());
        assert_eq!(canvas[4][4], 1);
        assert_eq!(canvas[5][3], 1);
        assert_eq!(canvas[5][2], 1);
        assert_eq!(canvas[6][1], 1);
        assert_eq!(canvas[7][0], 1);

        // 水平な線
        let mouse_pos = (8, 4);
        assert!(line
            .on_mouse_drag(&mut canvas, &canvas_size, &mut drawing, &mouse_pos)
            .is_ok());
        for i in 4..=8 {
            assert_eq!(canvas[4][i], 1);
            assert_eq!(drawing.get_grid().borrow().get_color(i, 4).unwrap(), 0);
        }

        let mouse_pos = (0, 4);
        assert!(line
            .on_mouse_drag(&mut canvas, &canvas_size, &mut drawing, &mouse_pos)
            .is_ok());
        for i in 0..=4 {
            assert_eq!(canvas[4][i], 1);
            assert_eq!(drawing.get_grid().borrow().get_color(i, 4).unwrap(), 0);
        }

        // 垂直な線
        let mouse_pos = (4, 0);
        assert!(line
            .on_mouse_drag(&mut canvas, &canvas_size, &mut drawing, &mouse_pos)
            .is_ok());
        for i in 0..=4 {
            assert_eq!(canvas[i][4], 1);
            assert_eq!(drawing.get_grid().borrow().get_color(4, i).unwrap(), 0);
        }

        let mouse_pos = (4, 8);
        assert!(line
            .on_mouse_drag(&mut canvas, &canvas_size, &mut drawing, &mouse_pos)
            .is_ok());
        for i in 4..=8 {
            assert_eq!(canvas[i][4], 1);
            assert_eq!(drawing.get_grid().borrow().get_color(4, i).unwrap(), 0);
        }
    }

    #[test]
    fn line_not_mouse_down_test() {
        let mut line = Line::new();
        let mut canvas = vec![vec![0; CANVAS_EDGE_SIZE]; CANVAS_EDGE_SIZE];
        let canvas_size = (CANVAS_EDGE_SIZE, CANVAS_EDGE_SIZE);
        let mut drawing = DrawingMock::new();

        let palette_binding = drawing.get_palette();
        let mut palette = palette_binding.borrow_mut();
        assert!(palette.add_color(ODColor::new(0, 0, 0)).is_ok());
        assert!(palette.select_color(1).is_ok());

        let grid_binding = drawing.get_grid();
        let mut grid = grid_binding.borrow_mut();
        assert!(grid.set_grid_width(CANVAS_EDGE_SIZE).is_ok());
        assert!(grid.set_grid_height(CANVAS_EDGE_SIZE).is_ok());

        // マウスダウンしていない状態でドラッグ
        let mouse_pos = (0, 0);
        assert!(line
            .on_mouse_drag(&mut canvas, &canvas_size, &mut drawing, &mouse_pos)
            .is_ok());
        assert_eq!(canvas[0][0], 0);
        assert_eq!(grid.get_color(0, 0).unwrap(), 0);

        let mouse_pos = (5, 5);
        assert!(line
            .on_mouse_drag(&mut canvas, &canvas_size, &mut drawing, &mouse_pos)
            .is_ok());
        for i in 0..10 {
            assert_eq!(canvas[i][i], 0);
            assert_eq!(grid.get_color(i, i).unwrap(), 0);
        }
    }
}
