use crate::common::color::ODColor;

fn convert_hex_char_into_num(hex_char: char) -> Result<u8, String> {
    match hex_char {
        '0' => Ok(0),
        '1' => Ok(1),
        '2' => Ok(2),
        '3' => Ok(3),
        '4' => Ok(4),
        '5' => Ok(5),
        '6' => Ok(6),
        '7' => Ok(7),
        '8' => Ok(8),
        '9' => Ok(9),
        'a' | 'A' => Ok(10),
        'b' | 'B' => Ok(11),
        'c' | 'C' => Ok(12),
        'd' | 'D' => Ok(13),
        'e' | 'E' => Ok(14),
        'f' | 'F' => Ok(15),
        _ => Err(format!(
            "convert_hex_char_into_num error. invalid char : {hex_char}.",
        )),
    }
}

fn convert_str_into_color_value(s: &str) -> Result<u8, String> {
    if s.len() != 2 {
        return Err(format!(
            "convert_str_into_color_value error. lenght of string is invalid : {s}."
        ));
    }

    let mut chars = s.chars();
    let c0 = chars.next().unwrap();
    let c1 = chars.next().unwrap();
    let color_val = convert_hex_char_into_num(c1)? + 16 * convert_hex_char_into_num(c0)?;

    Ok(color_val)
}

pub fn decode(paint_net_string: String, out: &mut Vec<ODColor>) -> Result<(), String> {
    for line in paint_net_string.lines() {
        if line.is_empty() {
            continue;
        }

        // ; から始まる行はコメント
        if line.starts_with(';') {
            continue;
        }

        // 色を含む行は AARRGGBB からなるので、8文字ぴったりになるはず
        if line.len() != 8 {
            return Err(format!(
                "paint.net decode error. contains invalid line : {line}."
            ));
        }

        // A は使用しないが、フォーマットに問題が無いか調べるために計算だけする
        let _ = convert_str_into_color_value(&line[0..=1])?;
        let r = convert_str_into_color_value(&line[2..=3])?;
        let g = convert_str_into_color_value(&line[4..=5])?;
        let b = convert_str_into_color_value(&line[6..=7])?;

        out.push(ODColor::new(r, g, b));
    }
    Ok(())
}

include!("tests/paint_net_codec_test.rs");
