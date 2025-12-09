use std::{
    borrow::Cow,
    os::raw::{c_char, c_void},
};

use crate::{ColorFormat, ContextVulkanInfo, TextureDescriptor};

/// An abstraction over graphics context (like OpenGL context).
///
/// It is reference counted, single threaded object.
pub trait Context: Clone {
    type DisplayListBuilder: crate::DisplayListBuilder<
        Paint = Self::Paint,
        ParagraphBuilder = Self::ParagraphBuilder,
        PathBuilder = Self::PathBuilder,
        Texture = Self::Texture,
    >;
    type Fonts: crate::Fonts;
    type FragmentShader: crate::FragmentShader;
    type Paint: crate::Paint<Texture = Self::Texture>;
    type ParagraphBuilder: crate::ParagraphBuilder<
        Texture = Self::Texture,
        Paint = Self::Paint,
        Fonts = Self::Fonts,
    >;
    type PathBuilder: crate::PathBuilder;
    type Surface: crate::Surface<Context = Self>;
    type Texture: crate::Texture;
    type VulkanSwapchain: crate::VulkanSwapchain;

    /// Create an OpenGL context.
    unsafe fn new_gl<F>(loadfn: F) -> Result<Self, &'static str>
    where
        F: FnMut(&str) -> *mut c_void;

    /// Create a Vulkan context.
    unsafe fn new_vulkan<F>(
        enable_validation: bool,
        proc_address_callback: F,
    ) -> Result<Self, &'static str>
    where
        F: FnMut(*mut c_void, *const c_char) -> *mut c_void;

    /// Get internal Vulkan handles managed by the given Vulkan context.
    fn get_vulkan_info(&self) -> Result<ContextVulkanInfo, &'static str>;

    /// Create a new Vulkan swapchain using a VkSurfaceKHR instance.
    unsafe fn create_new_vulkan_swapchain(
        &self,
        vulkan_surface_khr: *mut c_void,
    ) -> Option<Self::VulkanSwapchain>;

    /// Creates a new surface by wrapping an existing OpenGL framebuffer object.
    unsafe fn wrap_gl_framebuffer(
        &mut self,
        framebuffer_id: u32,
        width: u32,
        height: u32,
        color_format: ColorFormat,
    ) -> Result<Self::Surface, &'static str>;

    /// Creates a texture with an externally created OpenGL texture handle.
    unsafe fn adopt_gl_texture(
        &self,
        texture_handle: u32,
        descriptor: TextureDescriptor,
    ) -> Result<Self::Texture, &'static str>;

    /// Creates a new texture.
    unsafe fn create_texture(
        &self,
        contents: Cow<'static, [u8]>,
        descriptor: TextureDescriptor,
    ) -> Result<Self::Texture, &'static str>;

    /// Creates a new fragment shader using compiled program.
    unsafe fn create_fragment_shader(
        &self,
        program: Cow<'static, [u8]>,
    ) -> Result<Self::FragmentShader, &'static str>;
}
