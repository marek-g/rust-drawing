use crate::DipRect;

pub trait Path {
    fn get_bounds(&self) -> DipRect;
}
