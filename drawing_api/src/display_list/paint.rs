use crate::Color;

pub trait Paint {
    /// Sets the paint color for stroking or filling.
    fn set_color(&mut self, color: Color);
}
