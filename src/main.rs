use std::env;
use std::path::Path;

mod reader;
mod render;
mod utils;

use crate::reader::{read_exr_image, read_hdr_image};
use crate::render::render;

fn main() {
    let file =
        env::args().skip(1).next().expect("ERROR: A filepath to an image must be provided as a \
                                           commandline argument.");

    let filepath = Path::new(&file);

    if filepath.is_file() {
        if let Some(extension) = filepath.extension() {
            let image_buffer = match extension.to_str().unwrap() {
                "exr" => match read_exr_image(filepath) {
                    Ok(image) => image,
                    Err(error) => {
                        eprintln!("{:?}", error);
                        std::process::exit(1);
                    }
                },

                _ => match read_hdr_image(filepath) {
                    Ok(image) => image,
                    Err(error) => {
                        eprintln!("{:?}", error);
                        std::process::exit(1);
                    }
                },
            };

            if let Err(error) = render(image_buffer, filepath) {
                eprintln!("{}", error);
                std::process::exit(1);
            }
        }
    } else {
        eprintln!("ERROR: Could not read path. Please provide a valid HDR image path.");
        std::process::exit(1);
    }
}
