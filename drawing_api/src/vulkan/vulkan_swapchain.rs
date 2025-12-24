pub trait VulkanSwapchain: 'static {
    type Surface: crate::Surface;

    fn acquire_next_surface_new(&mut self) -> Result<Self::Surface, &'static str>;
}
