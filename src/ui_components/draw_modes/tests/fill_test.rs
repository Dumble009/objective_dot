#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::color::ODColor;
    use crate::mock::drawing_mock::DrawingMock;

    #[test]
    fn fill_entire_canvas_test() {
        let mut fill = Fill::new();
        let mut canvas = vec![vec![0; 5]; 5];
        let canvas_size = (5, 5);
        let mut drawing = DrawingMock::new();
        let palette = drawing.get_palette_mut();
        assert!(palette.add_color(ODColor::new(0, 0, 0)).is_ok());
        palette.select_color(1).unwrap();
        let grid = drawing.get_grid_mut();
        assert!(grid.set_grid_width(5).is_ok());
        assert!(grid.set_grid_height(5).is_ok());

        // Draw filled rectangle from (1,1) to (3,3)
        fill.on_mouse_down(&mut canvas, &canvas_size, &mut drawing, &(1, 1))
            .unwrap();

        fill.on_mouse_up(&mut canvas, &canvas_size, &mut drawing, &(1, 1))
            .unwrap();

        for y in 0..5 {
            for x in 0..5 {
                assert_eq!(canvas[y][x], 1);
                assert_eq!(drawing.get_grid().get_color(x, y).unwrap(), 1);
            }
        }
    }

    #[test]
    fn fill_rect_canvas_test() {
        // 囲まれた範囲を塗りつぶすテスト (斜めの四角形)
        let mut fill = Fill::new();
        let mut canvas = vec![vec![0; 5]; 5];
        let canvas_size = (5, 5);
        let mut drawing = DrawingMock::new();
        let palette = drawing.get_palette_mut();
        assert!(palette.add_color(ODColor::new(0, 0, 0)).is_ok());
        assert!(palette.add_color(ODColor::new(1, 1, 1)).is_ok());
        palette.select_color(1).unwrap();
        let grid = drawing.get_grid_mut();
        assert!(grid.set_grid_width(5).is_ok());
        assert!(grid.set_grid_height(5).is_ok());
        assert!(grid.set_color(2, 0, 2).is_ok());
        assert!(grid.set_color(1, 1, 2).is_ok());
        assert!(grid.set_color(0, 2, 2).is_ok());
        assert!(grid.set_color(1, 3, 2).is_ok());
        assert!(grid.set_color(2, 4, 2).is_ok());
        assert!(grid.set_color(3, 3, 2).is_ok());
        assert!(grid.set_color(4, 2, 2).is_ok());
        assert!(grid.set_color(3, 1, 2).is_ok());

        for y in 0..5 {
            for x in 0..5 {
                canvas[y][x] = grid.get_color(x, y).unwrap();
            }
        }

        fill.on_mouse_down(&mut canvas, &canvas_size, &mut drawing, &(2, 2))
            .unwrap();
        fill.on_mouse_up(&mut canvas, &canvas_size, &mut drawing, &(2, 2))
            .unwrap();

        for y in 0..5 {
            for x in 0..5 {
                if x == 2 && y == 2
                    || x == 1 && y == 2
                    || x == 2 && y == 1
                    || x == 2 && y == 3
                    || x == 3 && y == 2
                {
                    assert_eq!(canvas[y][x], 1);
                    assert_eq!(drawing.get_grid().get_color(x, y).unwrap(), 1);
                } else {
                    assert_ne!(canvas[y][x], 1);
                    assert_ne!(drawing.get_grid().get_color(x, y).unwrap(), 1);
                }
            }
        }
    }

    #[test]
    fn fill_one_dot_test() {
        // 一点だけのテスト
        let mut fill = Fill::new();
        let mut canvas = vec![vec![0; 5]; 5];
        let canvas_size = (5, 5);
        let mut drawing = DrawingMock::new();
        let palette = drawing.get_palette_mut();
        assert!(palette.add_color(ODColor::new(0, 0, 0)).is_ok());
        assert!(palette.add_color(ODColor::new(1, 1, 1)).is_ok());
        palette.select_color(1).unwrap();
        let grid = drawing.get_grid_mut();
        assert!(grid.set_grid_width(5).is_ok());
        assert!(grid.set_grid_height(5).is_ok());
        assert!(grid.set_color(1, 2, 2).is_ok());
        assert!(grid.set_color(2, 1, 2).is_ok());
        assert!(grid.set_color(3, 2, 2).is_ok());
        assert!(grid.set_color(2, 3, 2).is_ok());

        for y in 0..5 {
            for x in 0..5 {
                canvas[y][x] = grid.get_color(x, y).unwrap();
            }
        }

        fill.on_mouse_down(&mut canvas, &canvas_size, &mut drawing, &(2, 2))
            .unwrap();
        fill.on_mouse_up(&mut canvas, &canvas_size, &mut drawing, &(2, 2))
            .unwrap();

        for y in 0..5 {
            for x in 0..5 {
                if x == 2 && y == 2 {
                    assert_eq!(canvas[y][x], 1);
                    assert_eq!(drawing.get_grid().get_color(x, y).unwrap(), 1);
                } else {
                    assert_ne!(canvas[y][x], 1);
                    assert_ne!(drawing.get_grid().get_color(x, y).unwrap(), 1);
                }
            }
        }
    }

    #[test]
    fn fill_entire_canvas_with_same_color_test() {
        // 元と同じ色で塗りつぶした時のテスト
        let mut fill = Fill::new();
        let mut canvas = vec![vec![0; 5]; 5];
        let canvas_size = (5, 5);
        let mut drawing = DrawingMock::new();
        let palette = drawing.get_palette_mut();
        palette.select_color(0).unwrap();
        let grid = drawing.get_grid_mut();
        assert!(grid.set_grid_width(5).is_ok());
        assert!(grid.set_grid_height(5).is_ok());

        fill.on_mouse_down(&mut canvas, &canvas_size, &mut drawing, &(1, 1))
            .unwrap();
        fill.on_mouse_up(&mut canvas, &canvas_size, &mut drawing, &(1, 1))
            .unwrap();

        for y in 0..5 {
            for x in 0..5 {
                assert_eq!(canvas[y][x], 0);
                assert_eq!(drawing.get_grid().get_color(x, y).unwrap(), 0);
            }
        }
    }

    #[test]
    fn fill_out_of_canvas_test() {
        // 画面外をクリックしたときのテスト
        let mut fill = Fill::new();
        let mut canvas = vec![vec![0; 5]; 5];
        let canvas_size = (5, 5);
        let mut drawing = DrawingMock::new();
        let palette = drawing.get_palette_mut();
        assert!(palette.add_color(ODColor::new(0, 0, 0)).is_ok());
        palette.select_color(1).unwrap();
        let grid = drawing.get_grid_mut();
        assert!(grid.set_grid_width(5).is_ok());
        assert!(grid.set_grid_height(5).is_ok());

        fill.on_mouse_down(&mut canvas, &canvas_size, &mut drawing, &(6, 6))
            .unwrap();
        fill.on_mouse_up(&mut canvas, &canvas_size, &mut drawing, &(6, 6))
            .unwrap();

        for y in 0..5 {
            for x in 0..5 {
                assert_eq!(canvas[y][x], 0);
                assert_eq!(drawing.get_grid().get_color(x, y).unwrap(), 0);
            }
        }
    }
}
