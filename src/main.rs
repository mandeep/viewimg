use std::env;

mod render;
use render::render;

fn main() {
    let file = env::args().skip(1).next().expect("Error: Please provide a valid HDR image path.");

    render(file).unwrap();
}
