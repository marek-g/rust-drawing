use crate::generic::device::{convert_color, Color};

pub struct Paint {
    pub(crate) color: Color,
}

impl Paint {
    pub fn new() -> Self {
        Self {
            color: [0.0f32, 0.0f32, 0.0f32, 1.0f32],
        }
    }
}

impl drawing_api::Paint for Paint {
    fn set_color(&mut self, color: drawing_api::Color) {
        self.color = convert_color(&color);
    }
}
