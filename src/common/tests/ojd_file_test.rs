#[cfg(test)]
mod test {
    use crate::{
        common::{
            color::ODColor,
            ojd_file_codec::{decode, encode},
        },
        mock::{grid_mock::GridMock, palette_mock::PaletteMock},
    };

    use super::*;

    #[test]
    fn encode_decode_test() {
        let mut g1 = GridMock::new();
        let mut p1 = PaletteMock::new();

        let w = 20;
        let h = 20;
        g1.set_grid_width(w).unwrap();
        g1.set_grid_height(h).unwrap();

        let color1 = ODColor::new(0, 0, 0);
        let color2 = ODColor::new(1, 1, 1);
        let color3 = ODColor::new(2, 2, 2);
        p1.add_color(color1).unwrap();
        p1.add_color(color2).unwrap();
        p1.add_color(color3).unwrap();

        for x in 0..w {
            for y in 0..h {
                g1.set_color(x, y, (x + y * w) % p1.get_color_count())
                    .unwrap();
            }
        }

        let mut encoded = vec![];
        encode(&g1, &p1, &mut encoded).unwrap();

        let mut g2 = GridMock::new();
        let mut p2 = PaletteMock::new();
        decode(&encoded, &mut g2, &mut p2).unwrap();

        assert_eq_grid(&g1, &g2);
        assert_eq_palette(&p1, &p2);
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

    fn assert_eq_palette(p1: &dyn Palette, p2: &dyn Palette) {
        let color_count = p1.get_color_count();
        assert_eq!(color_count, p2.get_color_count());

        for i in 0..color_count {
            assert_eq!(p1.get_color(i).unwrap(), p2.get_color(i).unwrap());
        }
    }
}
