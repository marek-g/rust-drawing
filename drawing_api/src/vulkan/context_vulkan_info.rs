#[derive(Debug, Copy, Clone)]
pub struct ContextVulkanInfo {
    pub vk_instance: *mut ::std::os::raw::c_void,
    pub vk_physical_device: *mut ::std::os::raw::c_void,
    pub vk_logical_device: *mut ::std::os::raw::c_void,
    pub graphics_queue_family_index: u32,
    pub graphics_queue_index: u32,
}
