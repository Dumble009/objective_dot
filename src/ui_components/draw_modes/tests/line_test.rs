#[cfg(test)]
mod test {
    use super::*;

    use crate::common::color::ODColor;
    use crate::mock::drawing_mock::DrawingMock;

    #[test]
    fn line_draw_test1() {
        let mut line = Line::new();
        let mut canvas = vec![vec![0; 10]; 10];
        let canvas_size = (10, 10);
        let mut drawing = DrawingMock::new();
        let palette = drawing.get_palette_mut();
        assert!(palette.add_color(ODColor::new(0, 0, 0)).is_ok());
        assert!(palette.select_color(1).is_ok());
        let grid = drawing.get_grid_mut();
        assert!(grid.set_grid_width(10).is_ok());
        assert!(grid.set_grid_height(10).is_ok());

        // Start drawing a line
        let mouse_pos = (0, 0);
        assert!(line
            .on_mouse_down(&mut canvas, &canvas_size, &mut drawing, &mouse_pos)
            .is_ok());
        // 表示上は塗られるが、この時点ではまだ本当のグリッドには反映されない
        assert_eq!(canvas[0][0], 1);
        assert_eq!(drawing.get_grid_mut().get_color(0, 0).unwrap(), 0);

        // Dragging the mouse to draw the line
        let mouse_pos = (5, 5);
        assert!(line
            .on_mouse_drag(&mut canvas, &canvas_size, &mut drawing, &mouse_pos)
            .is_ok());
        for i in 0..6 {
            assert_eq!(canvas[i][i], 1);
            assert_eq!(drawing.get_grid_mut().get_color(i, i).unwrap(), 0);
        }

        // Finish drawing the line
        let mouse_pos = (9, 9);
        assert!(line
            .on_mouse_up(&mut canvas, &canvas_size, &mut drawing, &mouse_pos)
            .is_ok());
        // マウスを離した時点で本当のグリッドに反映される
        for i in 0..10 {
            assert_eq!(canvas[i][i], 1);
            assert_eq!(drawing.get_grid_mut().get_color(i, i).unwrap(), 1);
        }
    }

    #[test]
    fn line_draw_test2() {
        let mut line = Line::new();
        let mut canvas = vec![vec![0; 10]; 10];
        let canvas_size = (10, 10);
        let mut drawing = DrawingMock::new();
        let palette = drawing.get_palette_mut();
        assert!(palette.add_color(ODColor::new(0, 0, 0)).is_ok());
        assert!(palette.select_color(1).is_ok());
        let grid = drawing.get_grid_mut();
        assert!(grid.set_grid_width(10).is_ok());
        assert!(grid.set_grid_height(10).is_ok());

        // Start drawing a line
        let mouse_pos_start = (9, 9);
        assert!(line
            .on_mouse_down(&mut canvas, &canvas_size, &mut drawing, &mouse_pos_start)
            .is_ok());
        // 表示上は塗られるが、この時点ではまだ本当のグリッドには反映されない
        assert_eq!(canvas[9][9], 1);
        assert_eq!(drawing.get_grid_mut().get_color(9, 9).unwrap(), 0);

        // Dragging the mouse to draw the line
        let mouse_pos_drag = (5, 5);
        assert!(line
            .on_mouse_drag(&mut canvas, &canvas_size, &mut drawing, &mouse_pos_drag)
            .is_ok());
        for i in 0..5 {
            assert_eq!(canvas[9 - i][9 - i], 1);
            assert_eq!(drawing.get_grid_mut().get_color(9 - i, 9 - i).unwrap(), 0);
        }

        // Finish drawing the line
        let mouse_pos_end = (0, 0);
        assert!(line
            .on_mouse_up(&mut canvas, &canvas_size, &mut drawing, &mouse_pos_end)
            .is_ok());
        // マウスを離した時点で本当のグリッドに反映される
        for i in 0..10 {
            assert_eq!(canvas[i][i], 1);
            assert_eq!(drawing.get_grid_mut().get_color(i, i).unwrap(), 1);
        }
    }

    #[test]
    fn line_draw_test3() {
        let mut line = Line::new();
        let mut canvas = vec![vec![0; 10]; 10];
        let canvas_size = (10, 10);
        let mut drawing = DrawingMock::new();
        let palette = drawing.get_palette_mut();
        assert!(palette.add_color(ODColor::new(0, 0, 0)).is_ok());
        assert!(palette.select_color(1).is_ok());
        let grid = drawing.get_grid_mut();
        assert!(grid.set_grid_width(10).is_ok());
        assert!(grid.set_grid_height(10).is_ok());

        // Start drawing a line
        let mouse_pos = (9, 0);
        assert!(line
            .on_mouse_down(&mut canvas, &canvas_size, &mut drawing, &mouse_pos)
            .is_ok());
        // 表示上は塗られるが、この時点ではまだ本当のグリッドには反映されない
        assert_eq!(canvas[0][9], 1);
        assert_eq!(drawing.get_grid_mut().get_color(9, 0).unwrap(), 0);

        // Dragging the mouse to draw the line
        let mouse_pos = (5, 4);
        assert!(line
            .on_mouse_drag(&mut canvas, &canvas_size, &mut drawing, &mouse_pos)
            .is_ok());
        for i in 0..5 {
            println!("test {i}");
            assert_eq!(canvas[i][9 - i], 1);
            assert_eq!(drawing.get_grid_mut().get_color(9 - i, i).unwrap(), 0);
        }

        // Finish drawing the line
        let mouse_pos = (0, 9);
        assert!(line
            .on_mouse_up(&mut canvas, &canvas_size, &mut drawing, &mouse_pos)
            .is_ok());
        // マウスを離した時点で本当のグリッドに反映される
        for i in 0..10 {
            assert_eq!(canvas[i][9 - i], 1);
            assert_eq!(drawing.get_grid_mut().get_color(9 - i, i).unwrap(), 1);
        }
    }

    #[test]
    fn line_draw_test4() {
        let mut line = Line::new();
        let mut canvas = vec![vec![0; 10]; 10];
        let canvas_size = (10, 10);
        let mut drawing = DrawingMock::new();
        let palette = drawing.get_palette_mut();
        assert!(palette.add_color(ODColor::new(0, 0, 0)).is_ok());
        assert!(palette.select_color(1).is_ok());
        let grid = drawing.get_grid_mut();
        assert!(grid.set_grid_width(10).is_ok());
        assert!(grid.set_grid_height(10).is_ok());

        // Start drawing a line
        let mouse_pos_start = (0, 9);
        assert!(line
            .on_mouse_down(&mut canvas, &canvas_size, &mut drawing, &mouse_pos_start)
            .is_ok());
        // 表示上は塗られるが、この時点ではまだ本当のグリッドには反映されない
        assert_eq!(canvas[9][0], 1);
        assert_eq!(drawing.get_grid_mut().get_color(0, 9).unwrap(), 0);

        // Dragging the mouse to draw the line
        let mouse_pos_drag = (4, 5);
        assert!(line
            .on_mouse_drag(&mut canvas, &canvas_size, &mut drawing, &mouse_pos_drag)
            .is_ok());
        for i in 0..5 {
            assert_eq!(canvas[9 - i][i], 1);
            assert_eq!(drawing.get_grid_mut().get_color(i, 9 - i).unwrap(), 0);
        }

        // Finish drawing the line
        let mouse_pos_end = (9, 0);
        assert!(line
            .on_mouse_up(&mut canvas, &canvas_size, &mut drawing, &mouse_pos_end)
            .is_ok());
        // マウスを離した時点で本当のグリッドに反映される
        for i in 0..10 {
            assert_eq!(canvas[9 - i][i], 1);
            assert_eq!(drawing.get_grid_mut().get_color(i, 9 - i).unwrap(), 1);
        }
    }

    #[test]
    fn line_outside_canvas_test() {
        let mut line = Line::new();
        let mut canvas = vec![vec![0; 10]; 10];
        let canvas_size = (10, 10);
        let mut drawing = DrawingMock::new();
        let palette = drawing.get_palette_mut();
        assert!(palette.add_color(ODColor::new(0, 0, 0)).is_ok());
        assert!(palette.select_color(1).is_ok());
        let grid = drawing.get_grid_mut();
        assert!(grid.set_grid_width(10).is_ok());
        assert!(grid.set_grid_height(10).is_ok());

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
            assert_eq!(drawing.get_grid_mut().get_color(8 + i, 9 - i).unwrap(), 0);
        }

        assert!(line
            .on_mouse_up(&mut canvas, &canvas_size, &mut drawing, &mouse_pos)
            .is_ok());
        for i in 0..2 {
            assert_eq!(canvas[9 - i][8 + i], 1);
            assert_eq!(drawing.get_grid_mut().get_color(8 + i, 9 - i).unwrap(), 1);
        }
    }

    #[test]
    fn line_not_1x1_draw_test() {
        let mut line = Line::new();
        let mut canvas = vec![vec![0; 10]; 10];
        let canvas_size = (10, 10);
        let mut drawing = DrawingMock::new();
        let palette = drawing.get_palette_mut();
        assert!(palette.add_color(ODColor::new(0, 0, 0)).is_ok());
        assert!(palette.select_color(1).is_ok());
        let grid = drawing.get_grid_mut();
        assert!(grid.set_grid_width(10).is_ok());
        assert!(grid.set_grid_height(10).is_ok());

        let mouse_pos = (0, 0);
        assert!(line
            .on_mouse_down(&mut canvas, &canvas_size, &mut drawing, &mouse_pos)
            .is_ok());
        assert_eq!(canvas[0][0], 1);
        assert_eq!(drawing.get_grid_mut().get_color(0, 0).unwrap(), 0);

        let mouse_pos = (7, 3);
        assert!(line
            .on_mouse_drag(&mut canvas, &canvas_size, &mut drawing, &mouse_pos)
            .is_ok());
        for i in 0..4 {
            assert_eq!(canvas[i][i * 2], 1);
            assert_eq!(canvas[i][i * 2 + 1], 1);
            assert_eq!(drawing.get_grid_mut().get_color(i * 2, i).unwrap(), 0);
            assert_eq!(drawing.get_grid_mut().get_color(i * 2 + 1, i).unwrap(), 0);
        }

        let mouse_pos = (8, 5);
        assert!(line
            .on_mouse_drag(&mut canvas, &canvas_size, &mut drawing, &mouse_pos)
            .is_ok());
        assert_eq!(canvas[0][0], 1);
        assert_eq!(canvas[1][1], 1);
        assert_eq!(canvas[1][2], 1);
        assert_eq!(canvas[2][3], 1);
        assert_eq!(canvas[3][4], 1);
        assert_eq!(canvas[3][5], 1);
        assert_eq!(canvas[4][6], 1);
        assert_eq!(canvas[4][7], 1);
        assert_eq!(canvas[5][8], 1);
        assert_eq!(drawing.get_grid_mut().get_color(0, 0).unwrap(), 0);
        assert_eq!(drawing.get_grid_mut().get_color(1, 1).unwrap(), 0);
        assert_eq!(drawing.get_grid_mut().get_color(2, 1).unwrap(), 0);
        assert_eq!(drawing.get_grid_mut().get_color(3, 2).unwrap(), 0);
        assert_eq!(drawing.get_grid_mut().get_color(4, 3).unwrap(), 0);
        assert_eq!(drawing.get_grid_mut().get_color(5, 3).unwrap(), 0);
        assert_eq!(drawing.get_grid_mut().get_color(6, 4).unwrap(), 0);
        assert_eq!(drawing.get_grid_mut().get_color(7, 4).unwrap(), 0);
        assert_eq!(drawing.get_grid_mut().get_color(8, 5).unwrap(), 0);

        // 水平な線
        let mouse_pos = (9, 0);
        assert!(line
            .on_mouse_drag(&mut canvas, &canvas_size, &mut drawing, &mouse_pos)
            .is_ok());
        for i in 0..10 {
            assert_eq!(canvas[0][i], 1);
            assert_eq!(drawing.get_grid_mut().get_color(i, 0).unwrap(), 0);
        }

        // 垂直な線
        let mouse_pos = (0, 9);
        assert!(line
            .on_mouse_drag(&mut canvas, &canvas_size, &mut drawing, &mouse_pos)
            .is_ok());
        for i in 0..10 {
            assert_eq!(canvas[i][0], 1);
            assert_eq!(drawing.get_grid_mut().get_color(0, i).unwrap(), 0);
        }
    }

    #[test]
    fn line_not_mouse_down_test() {
        let mut line = Line::new();
        let mut canvas = vec![vec![0; 10]; 10];
        let canvas_size = (10, 10);
        let mut drawing = DrawingMock::new();
        let palette = drawing.get_palette_mut();
        assert!(palette.add_color(ODColor::new(0, 0, 0)).is_ok());
        assert!(palette.select_color(1).is_ok());
        let grid = drawing.get_grid_mut();
        assert!(grid.set_grid_width(10).is_ok());
        assert!(grid.set_grid_height(10).is_ok());

        // マウスダウンしていない状態でドラッグ
        let mouse_pos = (0, 0);
        assert!(line
            .on_mouse_drag(&mut canvas, &canvas_size, &mut drawing, &mouse_pos)
            .is_ok());
        assert_eq!(canvas[0][0], 0);
        assert_eq!(drawing.get_grid_mut().get_color(0, 0).unwrap(), 0);

        let mouse_pos = (5, 5);
        assert!(line
            .on_mouse_drag(&mut canvas, &canvas_size, &mut drawing, &mouse_pos)
            .is_ok());
        for i in 0..10 {
            assert_eq!(canvas[i][i], 0);
            assert_eq!(drawing.get_grid_mut().get_color(i, i).unwrap(), 0);
        }
    }
}
