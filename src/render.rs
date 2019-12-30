use image2::{io, Image, ImageBuf, Rgb, transform};
use pixels::{Error, Pixels, SurfaceTexture};
use pixels::wgpu::Surface;
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};
use winit_input_helper::WinitInputHelper;


pub fn render(file: String, width: u32, height: u32) -> Result<(), Error> {
    let image = create_image(file, width, height);
    let event_loop = EventLoop::new();
    let window = create_window(width, height, &event_loop);
    let surface = Surface::create(&window);
    let surface_texture = SurfaceTexture::new(width, height, surface);
    let mut pixels = Pixels::new(width, height, surface_texture).unwrap();

    let mut input = WinitInputHelper::new();

    event_loop.run(move |event, _, control_flow| {
        if let Event::WindowEvent { event: WindowEvent::RedrawRequested, .. } = event {
            draw_pixels(pixels.get_frame(), &image);
            pixels.render();
        }

        if input.update(event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
            }

            if let Some(size) = input.window_resized() {
                resize_pixels(&mut pixels, size);
            }

            window.request_redraw();
        }
    });
}


fn create_image(file: String, width: u32, height: u32) -> ImageBuf<u8, Rgb> {
    let image: ImageBuf<u8, Rgb> = io::read(file).unwrap();
    let mut resized_image = ImageBuf::new(width as usize, height as usize);
    transform::resize(&mut resized_image, &image, width as usize, height as usize);

    resized_image

}


fn create_window(width: u32, height: u32, event_loop: &EventLoop<()>) -> Window {
    let size = LogicalSize::new(width as f64, height as f64);
    let window = WindowBuilder::new()
        .with_title("viewimg")
        .with_inner_size(size)
        .with_min_inner_size(size)
        .with_resizable(true)
        .build(&event_loop)
        .unwrap();

    window
}


fn draw_pixels(frame: &mut [u8], image: &ImageBuf<u8, Rgb>) {
    let width = image.width();

    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        let x = i % width as usize;
        let y = i / width as usize;
        let rgba = [image.get(x, y, 0), image.get(x, y, 1), image.get(x, y, 2), 255];
        pixel.copy_from_slice(&rgba);
    }
}


fn resize_pixels(pixels: &mut Pixels, size: LogicalSize) {
    let new_width = size.width.round() as u32;
    let new_height = size.height.round() as u32;

    pixels.resize(new_width, new_height);
}
