#[cfg(test)]
mod test {
    use crate::common::bitmap::Bitmap;
    use crate::common::canvas_grid::Grid;
    use crate::common::palette::Palette;
    use crate::mock::drawing_mock::DrawingMock;

    #[test]
    fn simple_test() {
        let drawing1 = DrawingMock::new();
        drawing1.grid.borrow_mut().set_grid_width(2).unwrap();
        drawing1.grid.borrow_mut().set_grid_height(2).unwrap();
        drawing1
            .palette
            .borrow_mut()
            .change_color(0, crate::common::color::ODColor::new(0, 0, 0))
            .unwrap();
        drawing1
            .palette
            .borrow_mut()
            .add_color(crate::common::color::ODColor::new(255, 0, 0))
            .unwrap();
        drawing1
            .palette
            .borrow_mut()
            .add_color(crate::common::color::ODColor::new(0, 255, 0))
            .unwrap();
        drawing1
            .palette
            .borrow_mut()
            .add_color(crate::common::color::ODColor::new(0, 0, 255))
            .unwrap();
        drawing1.grid.borrow_mut().set_color(0, 0, 0).unwrap(); // Black
        drawing1.grid.borrow_mut().set_color(1, 0, 1).unwrap(); // Red
        drawing1.grid.borrow_mut().set_color(0, 1, 2).unwrap(); // Green
        drawing1.grid.borrow_mut().set_color(1, 1, 3).unwrap(); // Blue

        let bitmap_result = Bitmap::from_drawing(&drawing1, 1);
        assert!(bitmap_result.is_ok());

        let bitmap = bitmap_result.unwrap();
        let pixels = bitmap.pixels;
        assert_eq!(pixels.len(), 2 * 2 * 4); // 2x2 pixels, 4 bytes each
        assert_eq!(pixels[0..4], [0, 0, 0, 255]); // (0,0) Black
        assert_eq!(pixels[4..8], [255, 0, 0, 255]); // (1,0) Red
        assert_eq!(pixels[8..12], [0, 255, 0, 255]); // (0,1) Green
        assert_eq!(pixels[12..16], [0, 0, 255, 255]); // (1,1) Blue
        assert_eq!(bitmap.width, 2);
        assert_eq!(bitmap.height, 2);
    }

    #[test]
    fn multiple_pixels_test() {
        let drawing1 = DrawingMock::new();
        drawing1.grid.borrow_mut().set_grid_width(2).unwrap();
        drawing1.grid.borrow_mut().set_grid_height(2).unwrap();
        drawing1
            .palette
            .borrow_mut()
            .change_color(0, crate::common::color::ODColor::new(0, 0, 0))
            .unwrap();
        drawing1
            .palette
            .borrow_mut()
            .add_color(crate::common::color::ODColor::new(255, 0, 0))
            .unwrap();
        drawing1
            .palette
            .borrow_mut()
            .add_color(crate::common::color::ODColor::new(0, 255, 0))
            .unwrap();
        drawing1
            .palette
            .borrow_mut()
            .add_color(crate::common::color::ODColor::new(0, 0, 255))
            .unwrap();
        drawing1.grid.borrow_mut().set_color(0, 0, 0).unwrap(); // Black
        drawing1.grid.borrow_mut().set_color(1, 0, 1).unwrap(); // Red
        drawing1.grid.borrow_mut().set_color(0, 1, 2).unwrap(); // Green
        drawing1.grid.borrow_mut().set_color(1, 1, 3).unwrap(); // Blue

        let bitmap_result = Bitmap::from_drawing(&drawing1, 4);
        assert!(bitmap_result.is_ok());

        let bitmap = bitmap_result.unwrap();
        let pixels = bitmap.pixels;
        assert_eq!(pixels.len(), 8 * 8 * 4); // 8x8 pixels, 4 bytes each

        for i in 0..8 {
            for j in 0..8 {
                if i < 4 && j < 4 {
                    // 左上16ピクセル Black
                    assert_eq!(
                        pixels[(i * 32 + j * 4)..(i * 32 + j * 4) + 4],
                        [0, 0, 0, 255]
                    );
                } else if i < 4 && j >= 4 {
                    // 右上16ピクセル Red
                    assert_eq!(
                        pixels[(i * 32 + j * 4)..(i * 32 + j * 4) + 4],
                        [255, 0, 0, 255]
                    );
                } else if i >= 4 && j < 4 {
                    // 左下16ピクセル Green
                    assert_eq!(
                        pixels[(i * 32 + j * 4)..(i * 32 + j * 4) + 4],
                        [0, 255, 0, 255]
                    );
                } else {
                    // 右下16ピクセル Blue
                    assert_eq!(
                        pixels[(i * 32 + j * 4)..(i * 32 + j * 4) + 4],
                        [0, 0, 255, 255]
                    );
                }
            }
        }

        assert_eq!(bitmap.width, 8);
        assert_eq!(bitmap.height, 8);
    }
}
