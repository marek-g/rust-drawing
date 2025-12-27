use crate::VulkanSwapchain;

use super::SurfaceObject;

pub trait VulkanSwapchainObject {
    fn acquire_next_surface_new(&mut self) -> Result<Box<dyn SurfaceObject>, &'static str>;
}

impl<S: VulkanSwapchain> VulkanSwapchainObject for S {
    fn acquire_next_surface_new(&mut self) -> Result<Box<dyn SurfaceObject>, &'static str> {
        Ok(Box::new(self.acquire_next_surface_new()?))
    }
}
