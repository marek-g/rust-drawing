use std::{
    borrow::Cow,
    os::raw::c_void,
    sync::{Arc, Mutex},
};

use drawing_api::{Capabilities, ColorSource, Context, ImageFilter, TextureDescriptor};

use crate::{ImpellerSurface, ImpellerTexture};

#[derive(Clone)]
pub struct ImpellerContextVulkan {
    context: Arc<Mutex<impellers::Context>>,
}

unsafe impl Send for ImpellerContextVulkan {}
unsafe impl Sync for ImpellerContextVulkan {}

impl Context for ImpellerContextVulkan {
    type ColorSourceFragment = crate::ColorSourceFragment;

    type DisplayList = crate::DisplayList;

    type DisplayListBuilder = crate::DisplayListBuilder;

    type Fonts = crate::Fonts;

    type FragmentProgram = crate::ImpellerFragmentProgram;

    type ImageFilterFragment = crate::ImageFilterFragment;

    type Paint = crate::Paint;

    type ParagraphBuilder = crate::ParagraphBuilder;

    type PathBuilder = crate::PathBuilder;

    type Surface = ImpellerSurface;

    type Texture = ImpellerTexture;

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
            text_decorations: true,
            shadows: true,
            fragment_color_sources: true,
            fragment_image_filters: true,
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
            self.context.lock().unwrap().create_texture_with_rgba8(
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

    unsafe fn new_color_source_from_fragment_program(
        &self,
        frag_program: &Self::FragmentProgram,
        samplers: &[Self::Texture],
        uniform_data: &[u8],
    ) -> ColorSource<Self::Texture, Self::ColorSourceFragment> {
        let color_source = unsafe {
            self.context
                .lock()
                .unwrap()
                .new_color_source_from_fragment_program(
                    &frag_program.fragment_program,
                    &samplers
                        .iter()
                        .map(|s| s.texture.clone())
                        .collect::<Vec<_>>(),
                    uniform_data,
                )
        };
        let color_source_fragment = Self::ColorSourceFragment { color_source };
        ColorSource::Fragment {
            color_source: color_source_fragment,
        }
    }

    unsafe fn new_image_filter_from_fragment_program(
        &self,
        frag_program: &Self::FragmentProgram,
        samplers: &[Self::Texture],
        uniform_data: &[u8],
    ) -> ImageFilter<Self::ImageFilterFragment> {
        let image_filter = unsafe {
            self.context
                .lock()
                .unwrap()
                .new_image_filter_from_fragment_program(
                    &frag_program.fragment_program,
                    &samplers
                        .iter()
                        .map(|s| s.texture.clone())
                        .collect::<Vec<_>>(),
                    uniform_data,
                )
        };
        let image_filter_fragment = Self::ImageFilterFragment { image_filter };
        ImageFilter::Fragment {
            image_filter: image_filter_fragment,
        }
    }
}

impl drawing_api::ContextVulkan for ImpellerContextVulkan {
    type VulkanSwapchain = crate::VulkanSwapchain;

    unsafe fn new_vulkan<F>(
        enable_validation: bool,
        proc_address_callback: F,
    ) -> Result<Self, &'static str>
    where
        F: FnMut(*mut c_void, *const std::os::raw::c_char) -> *mut c_void,
    {
        unsafe {
            let context = Arc::new(Mutex::new(impellers::Context::new_vulkan(
                enable_validation,
                proc_address_callback,
            )?));
            Ok(Self { context })
        }
    }

    fn get_vulkan_info(&self) -> Result<drawing_api::ContextVulkanInfo, &'static str> {
        let vulkan_info = self.context.lock().unwrap().get_vulkan_info()?;
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
                .lock()
                .unwrap()
                .create_new_vulkan_swapchain(vulkan_surface_khr)
                .ok_or("impeller: cannot create new vulkan swapchain")?;
            Ok(Self::VulkanSwapchain { vk_swapchain })
        }
    }
}
