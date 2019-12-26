use std::env;

use image2::{io, Image, ImageBuf, Rgb};
use pixels::{Error, PixelsBuilder, SurfaceTexture};
use pixels::wgpu::{Surface, TextureFormat};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;


fn main() -> Result<(), Error> {
    let file = env::args().skip(1).next().expect("Error: Please provide a valid HDR image path.");
    let image: ImageBuf<u8, Rgb> = io::read(file).unwrap();
    let (width, height) = (image.width() as u32, image.height() as u32);

    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let size = LogicalSize::new(width as f64, height as f64);
    let window = WindowBuilder::new()
        .with_title("Patina")
        .with_inner_size(size)
        .with_min_inner_size(size)
        .build(&event_loop)
        .unwrap();

    let surface = Surface::create(&window);
    let surface_texture = SurfaceTexture::new(width, height, surface);
    let mut pixels = PixelsBuilder::new(height, width, surface_texture)
        .texture_format(TextureFormat::Rgba8UnormSrgb)
        .build()
        .unwrap();

    event_loop.run(move |event, _, control_flow| {
        if let Event::WindowEvent { event: WindowEvent::RedrawRequested, .. } = event {
            let frame = pixels.get_frame();

            for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
                let x = i % width as usize;
                let y = i / width as usize;
                let rgba = [image.get(x, y, 0), image.get(x, y, 1), image.get(x, y, 2), 255];
                pixel.copy_from_slice(&rgba);
            }

            pixels.render();
        }

        if input.update(event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        if let Some(size) = input.window_resized() {
            let size = size.to_physical(window.hidpi_factor());
            let width = size.width.round() as u32;
            let height = size.height.round() as u32;

            pixels.resize(width, height);
        }
    });
}
