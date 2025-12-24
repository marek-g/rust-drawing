use crate::PixelRect;

pub trait Path: 'static {
    fn get_bounds(&self) -> PixelRect;
}
