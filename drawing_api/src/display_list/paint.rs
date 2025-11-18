use crate::Color;

pub trait Paint {
    fn set_color(&mut self, color: Color);
}
