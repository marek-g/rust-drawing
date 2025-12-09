pub struct VulkanSwapchain {}

impl drawing_api::VulkanSwapchain for VulkanSwapchain {
    type Surface = crate::GlSurface;

    fn acquire_next_surface_new(&mut self) -> Result<Self::Surface, &'static str> {
        todo!()
    }
}
