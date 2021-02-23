#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct TexturedVertex {
    pub pos: [f32; 2],
    pub tex_coords: [f32; 2],
    pub color: [f32; 4],
}

impl TexturedVertex {
    pub fn new(pos: [f32; 2], tex_coords: [f32; 2], color: [f32; 4]) -> Self {
        TexturedVertex {
            pos,
            tex_coords,
            color,
        }
    }
}
