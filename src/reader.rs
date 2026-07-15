use std::path::Path;

use image::{ImageBuffer, ImageError, Rgb};

use crate::utils::{compensate, f32_to_u8};

pub fn read_image(filepath: &Path) -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>, ImageError> {
    let image = image::open(filepath)?;

    let extension = filepath
        .extension()
        .and_then(|extension| extension.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();

    let correct = match extension.as_str() {
        "hdr" => f32_to_u8,
        "exr" => compensate,
        _ => return Ok(image.into_rgb8()),
    };

    let f32_image = image.into_rgb32f();

    Ok(ImageBuffer::from_fn(
        f32_image.width(),
        f32_image.height(),
        |x, y| Rgb(f32_image.get_pixel(x, y).0.map(correct)),
    ))
}
