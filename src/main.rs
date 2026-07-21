use std::path::Path;

use clap::{crate_version, App, Arg};

pub mod macros;
pub mod options;
pub mod reader;
pub mod render;
pub mod utils;

use crate::options::RenderOptions;
use crate::reader::read_image;
use crate::render::render;

fn main() {
    let matches = App::new("viewimg")
        .version(crate_version!())
        .about("A simple HDR image viewer")
        .usage("viewimg <image> [FLAGS]")
        .after_help("SHORTCUTS:\n    Left Click    Drag the viewimg window\n    Q, Esc        Quit the program")
        // template needs to be updated anytime we change something in the CLI.
        // clap will no longer update the help output automatically
        .template("{bin} {version}\n{about}\n\nUSAGE:\n    {usage}\n\nARGS:\n{positionals}\n\nFLAGS:\n{flags}\n\n{after-help}\n")
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

    let options = RenderOptions {
        scale_image: !matches.is_present("native"),
        ..Default::default()
    };

    if let Err(error) = render(image_buffer, filepath, options) {
        exit!("{}", error);
    }
}
