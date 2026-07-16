pub struct RenderConfig {
    pub scale_image: bool,
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            scale_image: true,
        }
    }
}