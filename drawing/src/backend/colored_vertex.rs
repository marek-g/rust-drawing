#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct ColoredVertex {
    pub pos: [f32; 2],
    pub color: [f32; 4],
}

impl ColoredVertex {
    pub fn new(pos: [f32; 2], color: [f32; 4]) -> Self {
        ColoredVertex { pos, color }
    }
}
