pub fn normalize_f32(value: f32, minimum: f32, maximum: f32) -> f32 {
    (value - minimum) / (maximum - minimum)
}

pub fn clamp_f32(value: f32, lower_bound: f32, upper_bound: f32) -> f32 {
    let minimum = value.max(lower_bound);
    let maximum = value.min(upper_bound);

    minimum.min(maximum)
}

pub fn clamp_rgb(value: f32) -> f32 {
    value.clamp(0.0, 255.0)
}

pub fn gamma_correct(luminance: f32, gamma: f32) -> f32 {
    luminance.powf(1.0 / gamma)
}

/// References:
/// The original link is below however it is no longer in existence so the archive.is link was created.
/// https://www.openexr.com/using.html
/// https://archive.is/QnNQV
pub fn compensate(value: f32) -> u8 {
    let mut compensated_value = 0.0f32.max(value);

    // the below steps were taken from an older version of exrdisplay.
    // I believe the knee was solved for f = 0.16 and plugged into log(x * f + 1) / f.
    compensated_value *= 2.0f32.powf(2.47393);
    compensated_value = (compensated_value * 0.16 + 1.0).ln() / 0.16;
    compensated_value = gamma_correct(compensated_value, 2.2);

    // the last step calls for 2.0 ^ -3.5, however -1.0 results in proper
    // brightness on newer displays, or so it seems. Probably because the knee
    // is applied unconditionally.
    clamp_rgb(255.0 * compensated_value * 2.0f32.powf(-1.0)) as u8
}

pub fn f32_to_u8(pixel: f32) -> u8 {
    let corrected = gamma_correct(pixel.max(0.0), 2.2);
    clamp_rgb(corrected * 255.0).round() as u8
}
