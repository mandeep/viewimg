use std::env;
use std::path::Path;

use image2::io;

mod render;
use render::render;

fn main() {
    let file = env::args()
        .skip(1)
        .next()
        .expect("ERROR: A filepath to an image must be provided as a commandline argument.");

    let filepath = Path::new(&file);

    if filepath.is_file() {
        if let Ok(image) = io::read(filepath) {
            if let Err(error) = render(image, filepath) {
                eprintln!("{}", error);
                std::process::exit(1);
            }
        } else {
            eprintln!("ERROR: Failed to read image. Please make sure it is a valid image file.");
            std::process::exit(1);
        }
    } else {
        eprintln!("ERROR: Could not read path. Please provide a valid HDR image path.");
        std::process::exit(1);
    }

}
