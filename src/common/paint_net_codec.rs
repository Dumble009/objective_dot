use crate::common::color::ODColor;

fn convert_hex_char_into_num(hex_char: char) -> Result<u8, String> {
    match hex_char {
        '0' => return Ok(0),
        '1' => return Ok(1),
        '2' => return Ok(2),
        '3' => return Ok(3),
        '4' => return Ok(4),
        '5' => return Ok(5),
        '6' => return Ok(6),
        '7' => return Ok(7),
        '8' => return Ok(8),
        '9' => return Ok(9),
        'a' | 'A' => return Ok(10),
        'b' | 'B' => return Ok(11),
        'c' | 'C' => return Ok(12),
        'd' | 'D' => return Ok(13),
        'e' | 'E' => return Ok(14),
        'f' | 'F' => return Ok(15),
        _ => {
            return Err(String::from(format!(
                "convert_hex_char_into_num error. invalid char : {hex_char}.",
            )))
        }
    }
}

fn convert_str_into_color_value(s: &str) -> Result<u8, String> {
    if s.len() != 2 {
        return Err(String::from(format!(
            "convert_str_into_color_value error. lenght of string is invalid : {s}."
        )));
    }

    let c0 = s.chars().nth(0).unwrap();
    let c1 = s.chars().nth(1).unwrap();
    let color_val = convert_hex_char_into_num(c1)? + 16 * convert_hex_char_into_num(c0)?;

    Ok(color_val)
}

pub fn decode(paint_net_string: String, out: &mut Vec<ODColor>) -> Result<(), String> {
    for line in paint_net_string.lines() {
        if line.len() == 0 {
            continue;
        }

        // ; から始まる行はコメント
        if line.chars().nth(0).unwrap() == ';' {
            continue;
        }

        // 色を含む行は AARRGGBB からなるので、8文字ぴったりになるはず
        if line.len() != 8 {
            return Err(String::from(format!(
                "paint.net decode error. contains invalid line : {line}."
            )));
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
