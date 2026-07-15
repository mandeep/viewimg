use std::path::Path;

use clap::{crate_version, App, Arg};

pub mod macros;
pub mod reader;
pub mod render;
pub mod utils;

use crate::reader::read_image;
use crate::render::render;

fn main() {
    let matches = App::new("viewimg")
        .version(crate_version!())
        .arg(
            Arg::with_name("image")
                .help("The file path to the image to view")
                .index(1)
                .required(true),
        )
        .get_matches();

    let file = matches.value_of("image").unwrap();

    let filepath = Path::new(&file);

    if !filepath.is_file() {
       exit!("ERROR: Could not read path: {}. Please provide a valid image path.", filepath.display());
    }

    let image_buffer = match read_image(filepath) {
        Ok(image) => image,
        Err(error) => exit!("{:?}", error),
    };

    if let Err(error) = render(image_buffer, filepath) {
        exit!("{}", error);
    }
}
