use crate::PixelRect;

pub trait Path {
    fn get_bounds(&self) -> PixelRect;
}
