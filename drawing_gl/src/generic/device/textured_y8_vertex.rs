#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct TexturedY8Vertex {
    pub pos: [f32; 2],
    pub tex_coords: [f32; 2],
    pub color: [f32; 4],
}

impl TexturedY8Vertex {
    pub fn new(pos: [f32; 2], tex_coords: [f32; 2], color: [f32; 4]) -> Self {
        TexturedY8Vertex {
            pos,
            tex_coords,
            color,
        }
    }
}
