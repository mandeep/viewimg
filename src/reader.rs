use std::path::Path;

use exr::image::read_options;
use exr::image::rgba::{Image, Pixels};
use image2::{io, ImageBuf, Rgb};

use crate::utils::compensate;

fn extract_exr_data(image: &Image) -> Vec<u8> {
    let (width, height) = (image.resolution.0, image.resolution.1);

    let mut exr_data = vec![0u8; width as usize * height as usize * 3];

    for i in 0..(width * height) {
        let x = i % width as usize;
        let y = i / width as usize;
        let index = image.vector_index_of_first_pixel_component(exr::math::Vec2(x, y));

        let data: (f32, f32, f32) = match &image.data {
            Pixels::F32(data) => (data[index + 0], data[index + 1], data[index + 2]),
            Pixels::F16(data) => {
                (data[index + 0].to_f32(), data[index + 1].to_f32(), data[index + 2].to_f32())
            }
            Pixels::U32(data) => {
                (data[index + 0] as f32, data[index + 1] as f32, data[index + 2] as f32)
            }
        };

        exr_data[3 * i + 0] = compensate(data.0);
        exr_data[3 * i + 1] = compensate(data.1);
        exr_data[3 * i + 2] = compensate(data.2);
    }
    exr_data
}

pub fn read_exr_image(filepath: &Path) -> Result<ImageBuf<u8, Rgb>, exr::error::Error> {
    match Image::read_from_file(filepath, read_options::high()) {
        Ok(exr_image) => {
            let exr_data = extract_exr_data(&exr_image);
            let exr_image_buffer: ImageBuf<u8, Rgb> =
                ImageBuf::new_from(exr_image.resolution.0, exr_image.resolution.1, exr_data);
            Ok(exr_image_buffer)
        }

        Err(error) => Err(error),
    }
}

pub fn read_hdr_image(filepath: &Path) -> Result<ImageBuf<u8, Rgb>, image2::Error> {
    match io::read(filepath) {
        Ok(hdr_image_buffer) => Ok(hdr_image_buffer),

        Err(error) => Err(error),
    }
}
