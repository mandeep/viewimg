use std::env;

mod render;
use render::render;

fn main() {
    let file = env::args()
        .skip(1)
        .next()
        .expect("ERROR: A filepath to an image must be provided as a commandline argument.");


    render(file).unwrap();
}
