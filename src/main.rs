use std::path::Path;

use clap::{crate_version, App, Arg};

pub mod config;
pub mod macros;
pub mod reader;
pub mod render;
pub mod utils;

use crate::config::RenderConfig;
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
        .arg(
            Arg::with_name("native")
                .short("n")
                .long("native") // long name required for clap to treat this argument as a flag
                .help("View the image at its native resolution rather than scaling to fit the display")
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

    let config = RenderConfig {
        scale_image: !matches.is_present("native"),
        ..Default::default()
    };

    if let Err(error) = render(image_buffer, filepath, config) {
        exit!("{}", error);
    }
}
