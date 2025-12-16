use crate::ImpellerSurface;

pub struct VulkanSwapchain {
    pub(crate) vk_swapchain: impellers::VkSwapChain,
}

impl drawing_api::VulkanSwapchain for VulkanSwapchain {
    type Surface = crate::ImpellerSurface;

    fn acquire_next_surface_new(&mut self) -> Result<Self::Surface, &'static str> {
        Ok(ImpellerSurface {
            surface: self
                .vk_swapchain
                .acquire_next_surface_new()
                .ok_or("impeller: cannot acquire next vulkan surface")?,
        })
    }
}
