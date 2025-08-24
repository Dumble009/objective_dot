#[cfg(test)]
mod test {
    use crate::common::{color::ODColor, paint_net_codec::decode};

    #[test]
    fn decode_test() {
        let mut paint_net_string = String::new();
        paint_net_string.push_str(";paint.net Palette File\n");
        paint_net_string.push_str(";Downloaded from Lospec.com/palette-list\n");
        paint_net_string.push_str(";Palette Name: Ice Cream GB\n");
        paint_net_string.push_str(";Description: \n");
        paint_net_string.push_str(";Colors: 4\n");
        paint_net_string.push_str("FF7c3f58\n");
        paint_net_string.push_str("FFeb6b6f\n");
        paint_net_string.push_str("FFf9a875\n");
        paint_net_string.push_str("FFfff6d3\n");

        let mut colorset = vec![];
        let res = decode(paint_net_string, &mut colorset);
        assert!(res.is_ok());

        assert_eq!(4, colorset.len());
        assert_eq!(ODColor::new(0x7c, 0x3f, 0x58), colorset[0]);
        assert_eq!(ODColor::new(0xeb, 0x6b, 0x6f), colorset[1]);
        assert_eq!(ODColor::new(0xf9, 0xa8, 0x75), colorset[2]);
        assert_eq!(ODColor::new(0xff, 0xf6, 0xd3), colorset[3]);
    }

    #[test]
    fn decode_invalid_line_test() {
        // コメントにセミコロンがついていなかったらちゃんとエラーが返るか確認するテスト

        let mut paint_net_string = String::new();
        paint_net_string.push_str(";paint.net Palette File\n");
        paint_net_string.push_str(";Downloaded from Lospec.com/palette-list\n");
        paint_net_string.push_str("Palette Name: Ice Cream GB\n");
        paint_net_string.push_str(";Description: \n");
        paint_net_string.push_str(";Colors: 4\n");
        paint_net_string.push_str("FF7c3f58\n");
        paint_net_string.push_str("FFeb6b6f\n");
        paint_net_string.push_str("FFf9a875\n");
        paint_net_string.push_str("FFfff6d3\n");

        let mut colorset = vec![];
        let res = decode(paint_net_string, &mut colorset);
        assert!(res.is_err());
    }

    #[test]
    fn decode_invalid_color_test1() {
        // カラーコードが不正な場合にエラーが返るか確認するテスト

        let mut paint_net_string = String::new();
        paint_net_string.push_str(";paint.net Palette File\n");
        paint_net_string.push_str(";Downloaded from Lospec.com/palette-list\n");
        paint_net_string.push_str("Palette Name: Ice Cream GB\n");
        paint_net_string.push_str(";Description: \n");
        paint_net_string.push_str(";Colors: 4\n");
        paint_net_string.push_str("FF7g3f58\n"); // g が入っている
        paint_net_string.push_str("FFeb6b6f\n");
        paint_net_string.push_str("FFf9a875\n");
        paint_net_string.push_str("FFfff6d3\n");

        let mut colorset = vec![];
        let res = decode(paint_net_string, &mut colorset);
        assert!(res.is_err());
    }

    #[test]
    fn decode_invalid_color_test2() {
        // カラーコードが不正な場合にエラーが返るか確認するテスト

        let mut paint_net_string = String::new();
        paint_net_string.push_str(";paint.net Palette File\n");
        paint_net_string.push_str(";Downloaded from Lospec.com/palette-list\n");
        paint_net_string.push_str("Palette Name: Ice Cream GB\n");
        paint_net_string.push_str(";Description: \n");
        paint_net_string.push_str(";Colors: 4\n");
        paint_net_string.push_str("FF7c3f58\n");
        paint_net_string.push_str("eb6b6f\n"); // アルファが抜けている
        paint_net_string.push_str("FFf9a875\n");
        paint_net_string.push_str("FFfff6d3\n");

        let mut colorset = vec![];
        let res = decode(paint_net_string, &mut colorset);
        assert!(res.is_err());
    }
}
