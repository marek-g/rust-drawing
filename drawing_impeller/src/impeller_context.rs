use std::{borrow::Cow, cell::RefCell, os::raw::c_void, rc::Rc};

use drawing_api::{Capabilities, FragmentShader, TextureDescriptor};
use impellers::{ISize, PixelFormat};

use crate::{ImpellerSurface, ImpellerTexture};

#[derive(Clone)]
pub struct ImpellerContext {
    context: Rc<RefCell<impellers::Context>>,
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
        let mut capabilities = Capabilities {
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
                    capabilities.rect_clipping = false;
                    capabilities.path_clipping = false;
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
            let context = Rc::new(RefCell::new(impellers::Context::new_opengl_es(loadfn)?));
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
        unsafe {
            let context = Rc::new(RefCell::new(impellers::Context::new_vulkan(
                enable_validation,
                proc_address_callback,
            )?));
            Ok(Self { context })
        }
    }

    fn get_vulkan_info(&self) -> Result<drawing_api::ContextVulkanInfo, &'static str> {
        let vulkan_info = self.context.borrow().get_vulkan_info()?;
        Ok(drawing_api::ContextVulkanInfo {
            vk_instance: vulkan_info.vk_instance,
            vk_physical_device: vulkan_info.vk_physical_device,
            vk_logical_device: vulkan_info.vk_logical_device,
            graphics_queue_family_index: vulkan_info.graphics_queue_family_index,
            graphics_queue_index: vulkan_info.graphics_queue_index,
        })
    }

    unsafe fn create_new_vulkan_swapchain(
        &self,
        vulkan_surface_khr: *mut c_void,
    ) -> Result<Self::VulkanSwapchain, &'static str> {
        unsafe {
            let vk_swapchain = self
                .context
                .borrow()
                .create_new_vulkan_swapchain(vulkan_surface_khr)
                .ok_or("impeller: cannot create new vulkan swapchain")?;
            Ok(Self::VulkanSwapchain { vk_swapchain })
        }
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
                .borrow_mut()
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
                .borrow()
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
            self.context.borrow().create_texture_with_rgba8(
                contents,
                descriptor.width,
                descriptor.height,
            )?
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
        unsafe { Self::FragmentShader::new(program) }
    }
}
