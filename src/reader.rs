use std::path::Path;

use exr::image::read_options;
use exr::image::rgba::Image;
use image2::{io, ImageBuf, Rgb};

use crate::utils::extract_exr_data;

pub fn read_exr_image(filepath: &Path) -> Option<ImageBuf<u8, Rgb>> {
        match Image::read_from_file(filepath, read_options::high()) {
            Ok(exr_image) => {
                let exr_data = extract_exr_data(&exr_image);
                let exr_image_buffer: ImageBuf<u8, Rgb> =
                    ImageBuf::new_from(exr_image.resolution.0,
                                       exr_image.resolution.1,
                                       exr_data);
                return Some(exr_image_buffer);
            }

            Err(error) => eprintln!("{:?}", error)
        }

        None
}

pub fn read_hdr_image(filepath: &Path) -> Option<ImageBuf<u8, Rgb>> {
        match io::read(filepath) {
            Ok(hdr_image_buffer) => {
                return Some(hdr_image_buffer)
            }

            Err(error) => eprintln!("{}", error)
        }

        None
}
