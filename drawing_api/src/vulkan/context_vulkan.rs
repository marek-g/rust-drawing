use std::os::raw::{c_char, c_void};

use crate::Context;

use super::ContextVulkanInfo;

pub trait ContextVulkan: Context + Send + Sync + Clone + 'static {
    type VulkanSwapchain: crate::VulkanSwapchain;

    /// Creates a Vulkan context.
    unsafe fn new_vulkan<F>(
        enable_validation: bool,
        proc_address_callback: F,
    ) -> Result<Self, &'static str>
    where
        F: FnMut(*mut c_void, *const c_char) -> *mut c_void;

    /// Gets internal Vulkan handles managed by the given Vulkan context.
    fn get_vulkan_info(&self) -> Result<ContextVulkanInfo, &'static str>;

    /// Create a new Vulkan swapchain using a VkSurfaceKHR instance.
    unsafe fn create_new_vulkan_swapchain(
        &self,
        vulkan_surface_khr: *mut c_void,
    ) -> Result<Self::VulkanSwapchain, &'static str>;
}
