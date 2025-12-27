use std::os::raw::c_void;

use crate::{ContextVulkan, ContextVulkanInfo};

use super::{DrawingContextObject, VulkanSwapchainObject};

pub trait ContextVulkanObject: DrawingContextObject {
    /// Gets internal Vulkan handles managed by the given Vulkan context.
    fn get_vulkan_info(&self) -> Result<ContextVulkanInfo, &'static str>;

    /// Create a new Vulkan swapchain using a VkSurfaceKHR instance.
    unsafe fn create_new_vulkan_swapchain(
        &self,
        vulkan_surface_khr: *mut c_void,
    ) -> Result<Box<dyn VulkanSwapchainObject>, &'static str>;
}

impl<C: ContextVulkan> ContextVulkanObject for C {
    fn get_vulkan_info(&self) -> Result<ContextVulkanInfo, &'static str> {
        self.get_vulkan_info()
    }

    unsafe fn create_new_vulkan_swapchain(
        &self,
        vulkan_surface_khr: *mut c_void,
    ) -> Result<Box<dyn VulkanSwapchainObject>, &'static str> {
        unsafe {
            Ok(Box::new(
                self.create_new_vulkan_swapchain(vulkan_surface_khr)?,
            ))
        }
    }
}
