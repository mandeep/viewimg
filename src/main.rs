use std::env;
use std::path::Path;

use exr;
use image2;

mod render;
mod utils;

use crate::render::render;
use crate::utils::extract_exr_data;

fn main() {
    let file =
        env::args().skip(1).next().expect("ERROR: A filepath to an image must be provided as a \
                                           commandline argument.");

    let filepath = Path::new(&file);

    if filepath.is_file() {
        if let Some(extension) = filepath.extension() {
            if extension == "exr" {
                if let Ok(exr_image) =
                    exr::image::rgba::Image::read_from_file(filepath,
                                                            exr::image::read_options::high())
                {
                    let exr_data = extract_exr_data(&exr_image);
                    let exr_image_buffer: image2::ImageBuf<u8, image2::Rgb> =
                        image2::ImageBuf::new_from(exr_image.resolution.0,
                                                   exr_image.resolution.1,
                                                   exr_data);

                    if let Err(error) = render(exr_image_buffer, filepath) {
                        eprintln!("{}", error);
                        std::process::exit(1);
                    }
                } else {
                    eprintln!("ERROR: Failed to read OpenEXR image. Please make sure it is a \
                               valid image file.");
                    std::process::exit(1);
                }
            } else {
                if let Ok(hdr_image_buffer) = image2::io::read(filepath) {
                    if let Err(error) = render(hdr_image_buffer, filepath) {
                        eprintln!("{}", error);
                        std::process::exit(1);
                    }
                } else {
                    eprintln!("ERROR: Failed to read image. Please make sure it is a valid image \
                               file.");
                    std::process::exit(1);
                }
            }
        }
    } else {
        eprintln!("ERROR: Could not read path. Please provide a valid HDR image path.");
        std::process::exit(1);
    }
}
