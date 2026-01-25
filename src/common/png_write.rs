use crate::common::bitmap::Bitmap;
use png::Encoder;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

pub fn write_png(bitmap: &Bitmap, path_str: &str) -> Result<(), String> {
    let path = Path::new(path_str);
    let file = File::create(path).map_err(|e| format!("Failed to create file: {}", e))?;
    let mut w = BufWriter::new(file);

    let mut encoder = Encoder::new(&mut w, bitmap.width as u32, bitmap.height as u32);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2));
    // 色空間の設定
    // sRGBの色度を使用
    // https://ja.wikipedia.org/wiki/SRGB
    let source_chromaticities = png::SourceChromaticities::new(
        (0.3127, 0.3290),
        (0.6400, 0.3300),
        (0.3000, 0.6000),
        (0.1500, 0.0600),
    );
    encoder.set_source_chromaticities(source_chromaticities);
    let mut writer = encoder
        .write_header()
        .map_err(|e| format!("Failed to write PNG header: {}", e))?;

    writer
        .write_image_data(&bitmap.pixels)
        .map_err(|e| format!("Failed to write PNG image data: {}", e))?;

    Ok(())
}
