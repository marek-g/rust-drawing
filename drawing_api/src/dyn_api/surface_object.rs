use std::any::Any;

use crate::Surface;

use super::DisplayListObject;

pub trait SurfaceObject {
    /// Draws a display list on the surface.
    fn draw(&mut self, display_list: &Box<dyn DisplayListObject>) -> Result<(), &'static str>;

    /// Present the surface to the underlying window system (for Vulkan).
    fn present(self) -> Result<(), &'static str>;
}

impl<S: Surface> SurfaceObject for S {
    fn draw(&mut self, display_list: &Box<dyn DisplayListObject>) -> Result<(), &'static str> {
        let display_list = (display_list as &dyn Any)
            .downcast_ref::<S::DisplayList>()
            .unwrap();
        self.draw(display_list)
    }

    fn present(self) -> Result<(), &'static str> {
        self.present()
    }
}
