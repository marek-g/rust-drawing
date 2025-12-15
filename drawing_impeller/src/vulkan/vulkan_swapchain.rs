pub struct VulkanSwapchain {
    pub(crate) vk_swapchain: impellers::VkSwapChain,
}

impl drawing_api::VulkanSwapchain for VulkanSwapchain {
    type Surface = crate::ImpellerSurface;

    fn acquire_next_surface_new(&mut self) -> Result<Self::Surface, &'static str> {
        todo!()
    }
}
