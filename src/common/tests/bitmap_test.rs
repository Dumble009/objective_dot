#[cfg(test)]
mod test {
    use crate::common::bitmap::Bitmap;
    use crate::common::canvas_grid::Grid;
    use crate::common::drawing;
    use crate::common::palette::Palette;
    use crate::mock::drawing_mock::DrawingMock;

    #[test]
    fn from_drawing_test() {
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

        let bitmap_result = Bitmap::from_drawing(&drawing1);
        assert!(bitmap_result.is_ok());

        let bitmap = bitmap_result.unwrap();
        let pixels = bitmap.pixels;
        assert_eq!(pixels.len(), 2 * 2 * 4); // 2x2 pixels, 4 bytes each
        assert_eq!(pixels[0..4], [0, 0, 0, 255]); // (0,0) Black 0番目の色は背景色なので透明
        assert_eq!(pixels[4..8], [255, 0, 0, 255]); // (1,0) Red
        assert_eq!(pixels[8..12], [0, 255, 0, 255]); // (0,1) Green
        assert_eq!(pixels[12..16], [0, 0, 255, 255]); // (1,1) Blue
        assert_eq!(bitmap.width, 2);
        assert_eq!(bitmap.height, 2);
    }
}
