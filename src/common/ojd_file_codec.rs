use crate::common::color::ODColor;

use super::{drawing::Drawing, palette::PaletteColorIndex};
const MAGIC_O: u8 = 0x4f;
const MAGIC_J: u8 = 0x4a;
const MAGIC_D: u8 = 0x44;

macro_rules! append {
    ($v:expr, $val:expr) => {
        $v.append(&mut $val.to_be_bytes().to_vec());
    };
}

macro_rules! pop {
    ($v:expr, $pos:expr, $width:expr, $t:ty) => {{
        {
            let array: [u8; $width] = (&$v[$pos..($pos + $width)])
                .try_into()
                .expect("slice with incorrect length");

            $pos += $width;

            <$t>::from_be_bytes(array)
        }
    }};
}

pub fn encode(drawing: &dyn Drawing, out: &mut Vec<u8>) -> Result<(), String> {
    // マジックの追加
    out.push(MAGIC_O);
    out.push(MAGIC_J);
    out.push(MAGIC_D);

    // パレットのエンコード
    let palette = drawing.get_palette();
    let color_count = palette.get_color_count();
    append!(out, color_count);
    for i in 0..palette.get_color_count() {
        let color = palette.get_color(i)?.to_color32();
        append!(out, color.r());
        append!(out, color.g());
        append!(out, color.b());
    }

    // グリッドのエンコード
    let grid = drawing.get_grid();
    let width = grid.get_grid_width();
    let height = grid.get_grid_height();
    append!(out, width);
    append!(out, height);
    for y in 0..height {
        for x in 0..width {
            let color = grid.get_color(x, y)?;
            append!(out, color);
        }
    }

    Ok(())
}

pub fn decode(input: &[u8], drawing: &mut dyn Drawing) -> Result<(), String> {
    let mut pos = 0;
    // マジックの確認
    let magic1: u8 = pop!(input, pos, 1, u8);
    let magic2: u8 = pop!(input, pos, 1, u8);
    let magic3: u8 = pop!(input, pos, 1, u8);

    if magic1 != MAGIC_O || magic2 != MAGIC_J || magic3 != MAGIC_D {
        return Err(String::from("magic couldn't be found."));
    }

    // パレットのデコード
    let color_count: usize = pop!(input, pos, 8, usize);
    println!("color_count : {}", color_count);
    let palette = drawing.get_palette_mut();
    palette.reset();
    for i in 0..color_count {
        let r: u8 = pop!(input, pos, 1, u8);
        let g: u8 = pop!(input, pos, 1, u8);
        let b: u8 = pop!(input, pos, 1, u8);

        let color = ODColor::new(r, g, b);
        if palette.get_color_count() <= i {
            palette.add_color(color)?;
        } else {
            palette.change_color(i, color)?;
        }
    }

    // グリッドのデコード
    let width: usize = pop!(input, pos, 8, usize);
    let height: usize = pop!(input, pos, 8, usize);

    let grid = drawing.get_grid_mut();
    grid.set_grid_width(width)?;
    grid.set_grid_height(height)?;

    for y in 0..height {
        for x in 0..width {
            let color: PaletteColorIndex = pop!(input, pos, 8, PaletteColorIndex);
            grid.set_color(x, y, color)?;
        }
    }

    Ok(())
}

include!("tests/ojd_file_test.rs");
