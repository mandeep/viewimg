use exr::image::rgba::{Image, Pixels};
use exr::prelude::f16;

pub fn find_min_max(image: &Image) -> (f32, f32) {
    let min_max = match &image.data {
        Pixels::F32(data) => (data.iter().cloned().fold(0.0 / 0.0, f32::min),
                              data.iter().cloned().fold(0.0 / 0.0, f32::max)),
        Pixels::F16(data) => {
            let mut minimum = f16::MAX;
            let mut maximum = f16::MIN;

            for value in data {
                if *value < minimum {
                    minimum = *value;
                } else if *value > maximum {
                    maximum = *value;
                }
            }

            (minimum.to_f32(), maximum.to_f32())
        }
        Pixels::U32(data) => {
            ((*data.iter().min().unwrap()) as f32, (*data.iter().max().unwrap()) as f32)
        }
    };

    min_max
}

pub fn normalize_f32(value: f32, minimum: f32, maximum: f32) -> f32 {
    (value - minimum) / (maximum - minimum)
}

pub fn clamp_f32(value: f32, lower_bound: f32, upper_bound: f32) -> f32 {
    let minimum = value.max(lower_bound);
    let maximum = value.min(upper_bound);

    minimum.min(maximum)
}

pub fn clamp_rgb(value: f32) -> f32 {
    value.min(255.0).max(0.0)
}

pub fn gamma_correct(luminance: f32, gamma: f32) -> f32 {
    luminance.powf(1.0 / gamma)
}

/// Reference: https://www.openexr.com/using.html
pub fn compensate(value: f32) -> u8 {
    let mut compensated_value = 0.0f32.max(value);

    compensated_value *= 2.0f32.powf(2.47393);
    compensated_value = (compensated_value * 0.16 + 1.0).ln() / 0.16;
    compensated_value = gamma_correct(compensated_value, 2.2);

    clamp_rgb(255.0 * compensated_value * 2.0f32.powf(-3.5 / 2.2)) as u8
}
