use std::path::Path;

use image::{ImageBuffer, Rgb};
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Icon, Window, WindowBuilder};
use winit_input_helper::WinitInputHelper;

use crate::exit;

const ICON_BYTES: &[u8] = include_bytes!("../assets/icon.png");
const TASKBAR_MARGIN: u32 = 100;

enum Sizing {
    Native,
    Scaled { original: (u32, u32) },
}

fn load_icon() -> Icon {
    let img = image::load_from_memory(ICON_BYTES).unwrap().into_rgba8();
    let (width, height) = img.dimensions();

    Icon::from_rgba(img.into_raw(), width, height).expect("Failed to create icon.")
}

pub fn render(image: ImageBuffer<Rgb<u8>, Vec<u8>>, file: &Path) -> Result<(), Error> {
    let event_loop = EventLoop::new();

    let ((width, height), sizing) = calculate_dimensions(&image, &event_loop);

    if width == 0 || height == 0 {
        exit!("Failed to get image dimensions");
    }

    let image_to_render = match sizing {
        Sizing::Scaled { original: (original_width, original_height) } => {
            println!(
                "Note: the image was scaled down from {}x{} to {}x{} to fit the display.",
                original_width, original_height, width, height
            );
            resize_image(&image, width, height)
        }
        Sizing::Native => image,
    };

    let window = create_window(width, height, &event_loop, file);
    let mut pixels = create_pixel_buffer(&window, width, height);
    let mut input = WinitInputHelper::new();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::RedrawRequested(_) => {
                draw_pixels(pixels.frame_mut(), &image_to_render);
                if let Err(err) = pixels.render() {
                    exit!("{}", err);
                }
            }
            _ => {
                if input.update(&event) {
                    if input.key_pressed(VirtualKeyCode::Escape)
                        || input.key_pressed(VirtualKeyCode::Q)
                        || input.quit()
                    {
                        *control_flow = ControlFlow::Exit;
                    }

                    if input.mouse_pressed(0) {
                        if let Err(error) = window.drag_window() {
                            eprintln!("Failed to drag window: {error}");
                        }
                    }
                }
            }
        }
    });
}

fn calculate_dimensions(image: &ImageBuffer<Rgb<u8>, Vec<u8>>, event_loop: &EventLoop<()>) -> ((u32, u32), Sizing) {
    let (image_width, image_height) = image.dimensions();
    let monitor_size = event_loop.primary_monitor().unwrap().size();

    let usable_width = monitor_size.width;
    let usable_height = monitor_size.height.saturating_sub(TASKBAR_MARGIN);

    if image_width <= usable_width && image_height <= usable_height {
        return ((image_width, image_height), Sizing::Native);
    }

    let width_scale = usable_width as f32 / image_width as f32;
    let height_scale = usable_height as f32 / image_height as f32;
    let scale = width_scale.min(height_scale);

    let new_width = (image_width as f32 * scale).round() as u32;
    let new_height = (image_height as f32 * scale).round() as u32;

    ((new_width, new_height),  Sizing::Scaled { original: (image_width, image_height) })
}

fn resize_image(image: &ImageBuffer<Rgb<u8>, Vec<u8>>, width: u32, height: u32) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let resized_image = image::imageops::resize(image, width, height, image::imageops::FilterType::Lanczos3);

    resized_image
}

fn create_window(width: u32, height: u32, event_loop: &EventLoop<()>, file: &Path) -> Window {
    let size = PhysicalSize::new(width, height);

    let filename = file
        .file_name()
        .unwrap_or(std::ffi::OsStr::new("viewimg"))
        .to_str()
        .unwrap_or("viewimg");

    let window_icon = load_icon();

    let monitor_size = event_loop.primary_monitor().unwrap().size();
    let usable_height = monitor_size.height.saturating_sub(TASKBAR_MARGIN);
    let position = PhysicalPosition::new(
        monitor_size.width.saturating_sub(width) / 2,
        usable_height.saturating_sub(height) / 2,
    );

    let window = WindowBuilder::new()
        .with_title(filename)
        .with_window_icon(Some(window_icon))
        .with_inner_size(size)
        .with_position(position)
        .with_resizable(true)
        .with_decorations(false)
        .build(&event_loop)
        .unwrap();

    window
}

fn create_pixel_buffer(window: &Window, width: u32, height: u32) -> Pixels {
    let surface_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(surface_size.width, surface_size.height, window);
    Pixels::new(width, height, surface_texture).unwrap()
}

fn draw_pixels(frame: &mut [u8], image: &ImageBuffer<Rgb<u8>, Vec<u8>>) {
    for (frame_buffer, image_buffer) in frame.chunks_exact_mut(4).zip(image.as_raw().chunks_exact(3)) {
        frame_buffer[..3].copy_from_slice(image_buffer);
        frame_buffer[3] = 255;
    }
}