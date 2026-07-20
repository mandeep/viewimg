pub struct RenderOptions {
    pub scale_image: bool,
}

impl Default for RenderOptions {
    fn default() -> Self {
        Self {
            scale_image: true,
        }
    }
}