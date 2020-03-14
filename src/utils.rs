use exr::image::rgba::{Image, Pixels};

pub fn extract_exr_data(image: &Image) -> Vec<u8> {
    let (width, height) = (image.resolution.0, image.resolution.1);

    let mut exr_data = vec![0u8; width as usize * height as usize * 3];

    for i in 0..(width * height) {
        let x = i % width as usize;
        let y = i / width as usize;
        let index = image.vector_index_of_first_pixel_component(exr::math::Vec2(x, y));

        match &image.data {
            Pixels::F32(data) => {
                exr_data[3 * i + 0] =
                    (gamma_correct(clamp_f32(data[index + 0], 0.0, 1.0), 2.0) * 255.0) as u8;
                exr_data[3 * i + 1] =
                    (gamma_correct(clamp_f32(data[index + 1], 0.0, 1.0), 2.0) * 255.0) as u8;
                exr_data[3 * i + 2] =
                    (gamma_correct(clamp_f32(data[index + 2], 0.0, 1.0), 2.0) * 255.0) as u8;
            }

            Pixels::F16(data) => {
                exr_data[3 * i + 0] = (gamma_correct(clamp_f32(data[index + 0].to_f32(), 0.0, 1.0),
                                                     2.0)
                                       * 255.0) as u8;
                exr_data[3 * i + 1] = (gamma_correct(clamp_f32(data[index + 1].to_f32(), 0.0, 1.0),
                                                     2.0)
                                       * 255.0) as u8;
                exr_data[3 * i + 2] = (gamma_correct(clamp_f32(data[index + 2].to_f32(), 0.0, 1.0),
                                                     2.0)
                                       * 255.0) as u8;
            }

            _ => unimplemented!(),
        }
    }
    exr_data
}

pub fn clamp_f32(value: f32, lower_bound: f32, upper_bound: f32) -> f32 {
    let minimum = value.max(lower_bound);
    let maximum = value.min(upper_bound);

    minimum.min(maximum)
}

pub fn clamp_rgb(value: f32) -> f32 { value.min(255.0).max(0.0) }

pub fn gamma_correct(luminance: f32, gamma: f32) -> f32 { luminance.powf(1.0 / gamma) }
