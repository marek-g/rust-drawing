pub enum GraphicsApi {
    OpenGL { major: u32, minor: u32 },
    OpenGLES { major: u32, minor: u32 },
    Vulkan { major: u32, minor: u32 },
}

/// Drawing backend capabilities.
pub struct Capabilities {
    pub transformations: bool,
    pub layers: bool,
    pub rect_clipping: bool,
    pub path_clipping: bool,
    pub color_filters: bool,
    pub image_filters: bool,
    pub mask_filters: bool,
    pub textures: bool,
    pub text_metrics: bool,
    pub text_decorations: bool,
    pub shadows: bool,
    pub fragment_color_sources: bool,
    pub fragment_image_filters: bool,
}
