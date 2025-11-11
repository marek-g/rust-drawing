pub type Color = [f32; 4]; // R, G, B, A

pub enum ColorFormat {
    // for color images, 24-bit color with 8-bit alpha channel
    RGBA,

    // 8-bit channel, for use with monochromatic textures (like fonts)
    Y8,
}
