#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_grid_size() {
        let mut drawing = ObjectDrawing::new();
        let w = drawing.get_grid_width();
        let h = drawing.get_grid_height();
        assert_eq!(w, 10);
        assert_eq!(h, 10);

        drawing.set_grid_width(20).unwrap();
        drawing.set_grid_height(30).unwrap();
        let w = drawing.get_grid_width();
        let h = drawing.get_grid_height();
        assert_eq!(w, 20);
        assert_eq!(h, 30);
    }

    #[test]
    fn test_layer() {
        let mut drawing = ObjectDrawing::new();
        assert_eq!(drawing.get_layer_count(), 1);

        drawing.add_grid_layer();
        assert_eq!(drawing.get_layer_count(), 2);

        drawing.add_grid_layer();
        assert_eq!(drawing.get_layer_count(), 3);

        drawing.set_grid_width(15).unwrap();
        drawing.set_grid_height(25).unwrap();

        assert_eq!(
            drawing.get_grid_layer(1).unwrap().borrow().get_grid_width(),
            15
        );
        assert_eq!(
            drawing
                .get_grid_layer(2)
                .unwrap()
                .borrow()
                .get_grid_height(),
            25
        );

        assert!(drawing.set_active_layer_idx(1).is_ok());
        drawing
            .get_grid_layer(1)
            .unwrap()
            .borrow_mut()
            .set_color(0, 0, 1)
            .unwrap();
        drawing
            .get_grid_layer(1)
            .unwrap()
            .borrow_mut()
            .set_color(1, 1, 1)
            .unwrap();

        assert!(drawing.set_active_layer_idx(2).is_ok());

        drawing
            .get_grid_layer(2)
            .unwrap()
            .borrow_mut()
            .set_color(1, 1, 2)
            .unwrap();

        let color00 = drawing.get_grid().unwrap().get_color(0, 0).unwrap();
        assert_eq!(color00, 1);

        let color11 = drawing.get_grid().unwrap().get_color(1, 1).unwrap();
        assert_eq!(color11, 2);
    }
}
