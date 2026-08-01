#[cfg(test)]
mod test {
    use crate::{
        common::{
            canvas_grid::Grid,
            color::ODColor,
            drawing::Drawing,
            ojd_file_codec::{decode, encode},
            palette::Palette,
        },
        mock::drawing_mock::DrawingMock,
    };

    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn encode_decode_test() {
        let drawing1 = DrawingMock::new();

        let w = 20;
        let h = 20;
        drawing1.grid.borrow_mut().set_grid_width(w).unwrap();
        drawing1.grid.borrow_mut().set_grid_height(h).unwrap();

        let color1 = ODColor::new(0, 0, 0);
        let color2 = ODColor::new(1, 1, 1);
        let color3 = ODColor::new(2, 2, 2);
        drawing1.palette.borrow_mut().add_color(color1).unwrap();
        drawing1.palette.borrow_mut().add_color(color2).unwrap();
        drawing1.palette.borrow_mut().add_color(color3).unwrap();

        for x in 0..w {
            for y in 0..h {
                drawing1
                    .grid
                    .borrow_mut()
                    .set_color(
                        x,
                        y,
                        (x + y * w) % drawing1.palette.borrow().get_color_count(),
                    )
                    .unwrap();
            }
        }

        let mut encoded = vec![];
        encode(&drawing1, &mut encoded).unwrap();

        let mut drawing2 = Rc::new(RefCell::new(DrawingMock::new()));
        drawing2.borrow_mut().set_grid_width(w + 10).unwrap();
        drawing2.borrow_mut().set_grid_height(h + 10).unwrap();
        drawing2
            .borrow_mut()
            .palette
            .borrow_mut()
            .add_color(color1)
            .unwrap();
        drawing2
            .borrow_mut()
            .palette
            .borrow_mut()
            .add_color(color2)
            .unwrap();
        drawing2
            .borrow_mut()
            .palette
            .borrow_mut()
            .add_color(color3)
            .unwrap();

        let color4 = ODColor::new(3, 3, 3);
        drawing2
            .borrow_mut()
            .palette
            .borrow_mut()
            .add_color(color4)
            .unwrap();
        decode(&encoded, drawing2.clone()).unwrap();

        assert_eq_grid(
            &*drawing1.grid.borrow(),
            &*drawing2.borrow().get_grid().unwrap().borrow(),
        );
        assert_eq_palette(drawing1.palette, drawing2.borrow_mut().get_palette());
    }

    fn assert_eq_grid(g1: &dyn Grid, g2: &dyn Grid) {
        let w = g1.get_grid_width();
        let h = g1.get_grid_height();
        assert_eq!(w, g2.get_grid_width());
        assert_eq!(h, g2.get_grid_height());

        for x in 0..w {
            for y in 0..h {
                assert_eq!(g1.get_color(x, y).unwrap(), g2.get_color(x, y).unwrap());
            }
        }
    }

    fn assert_eq_palette(p1: Rc<RefCell<dyn Palette>>, p2: Rc<RefCell<dyn Palette>>) {
        let color_count = p1.borrow().get_color_count();
        assert_eq!(color_count, p2.borrow().get_color_count());

        for i in 0..color_count {
            assert_eq!(
                p1.borrow().get_color(i).unwrap(),
                p2.borrow().get_color(i).unwrap()
            );
        }
    }
}
