use std::{borrow::Cow, os::raw::c_void, ptr::fn_addr_eq};

use drawing_api::{Capabilities, TextureDescriptor};
use impellers::{ISize, PixelFormat};

use crate::{ImpellerSurface, ImpellerTexture};

#[derive(Clone)]
pub struct ImpellerContext {
    context: impellers::Context,
}

impl drawing_api::Context for ImpellerContext {
    type DisplayListBuilder = crate::DisplayListBuilder;

    type Fonts = crate::Fonts;

    type FragmentShader = crate::ImpellerFragmentShader;

    type Paint = crate::Paint;

    type ParagraphBuilder = crate::ParagraphBuilder;

    type PathBuilder = crate::PathBuilder;

    type Surface = ImpellerSurface;

    type Texture = ImpellerTexture;

    type VulkanSwapchain = crate::VulkanSwapchain;

    fn get_capabilities(api: drawing_api::GraphicsApi) -> Option<drawing_api::Capabilities> {
        let capabilities = Capabilities {
            transformations: true,
            layers: true,
            rect_clipping: true,
            path_clipping: true,
            color_filters: true,
            image_filters: true,
            mask_filters: true,
            textures: true,
            text_metrics: true,
            text_decorations: false, // TODO: add when impellers is updated
            shadows: true,
            fragment_shaders: false, // TODO: add when impellers is updated
        };
        match api {
            drawing_api::GraphicsApi::OpenGL { major, minor } => {
                if major >= 4 || major == 3 && minor >= 1 {
                    Some(capabilities)
                } else {
                    None
                }
            }
            drawing_api::GraphicsApi::OpenGLES { major, minor: _ } => {
                if major >= 2 {
                    Some(capabilities)
                } else {
                    None
                }
            }
            drawing_api::GraphicsApi::Vulkan { major, minor } => {
                if major >= 2 || major == 1 && minor >= 1 {
                    Some(capabilities)
                } else {
                    None
                }
            }
        }
    }

    unsafe fn new_gl<F>(loadfn: F) -> Result<Self, &'static str>
    where
        F: FnMut(&str) -> *mut c_void,
    {
        unsafe {
            let context = impellers::Context::new_opengl_es(loadfn)?;
            Ok(Self { context })
        }
    }

    unsafe fn new_vulkan<F>(
        enable_validation: bool,
        proc_address_callback: F,
    ) -> Result<Self, &'static str>
    where
        F: FnMut(*mut c_void, *const std::os::raw::c_char) -> *mut c_void,
    {
        todo!()
    }

    fn get_vulkan_info(&self) -> Result<drawing_api::ContextVulkanInfo, &'static str> {
        todo!()
    }

    unsafe fn create_new_vulkan_swapchain(
        &self,
        vulkan_surface_khr: *mut c_void,
    ) -> Option<Self::VulkanSwapchain> {
        todo!()
    }

    unsafe fn wrap_gl_framebuffer(
        &mut self,
        framebuffer_id: u32,
        width: u32,
        height: u32,
        color_format: drawing_api::ColorFormat,
    ) -> Result<Self::Surface, &'static str> {
        if color_format != drawing_api::ColorFormat::RGBA {
            return Err("color format not supported!");
        }

        unsafe {
            let surface = self
                .context
                .wrap_fbo(
                    framebuffer_id as u64,
                    PixelFormat::RGBA8888,
                    ISize::new(width as i64, height as i64),
                )
                .ok_or("ddd")?;
            Ok(ImpellerSurface { surface })
        }
    }

    unsafe fn adopt_gl_texture(
        &self,
        texture_handle: u32,
        descriptor: TextureDescriptor,
    ) -> Result<Self::Texture, &'static str> {
        if descriptor.color_format != drawing_api::ColorFormat::RGBA {
            return Err("color format not supported!");
        }

        // TODO: ensure texture is destroyed before context
        let texture = unsafe {
            self.context
                .adopt_opengl_texture(
                    descriptor.width,
                    descriptor.height,
                    descriptor.mip_count,
                    texture_handle as u64,
                )
                .ok_or("")?
        };

        Ok(ImpellerTexture {
            texture,
            descriptor,
        })
    }

    unsafe fn create_texture(
        &self,
        contents: Cow<'static, [u8]>,
        descriptor: TextureDescriptor,
    ) -> Result<Self::Texture, &'static str> {
        if descriptor.color_format != drawing_api::ColorFormat::RGBA {
            return Err("color format not supported!");
        }

        // TODO: ensure texture is destroyed before context
        let texture = unsafe {
            self.context
                .create_texture_with_rgba8(contents, descriptor.width, descriptor.height)?
        };
        Ok(ImpellerTexture {
            texture,
            descriptor,
        })
    }

    unsafe fn create_fragment_shader(
        &self,
        program: Cow<'static, [u8]>,
    ) -> Result<Self::FragmentShader, &'static str> {
        todo!()
    }
}
