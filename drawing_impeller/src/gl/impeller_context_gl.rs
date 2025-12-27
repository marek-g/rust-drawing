use std::{borrow::Cow, cell::RefCell, os::raw::c_void, rc::Rc};

use drawing_api::{Capabilities, ColorSource, GraphicsApi, TextureDescriptor};

use crate::{ImpellerSurface, ImpellerTexture};

#[derive(Clone)]
pub struct ImpellerContextGl {
    context: Rc<RefCell<impellers::Context>>,
}

impl drawing_api::Context for ImpellerContextGl {
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

    fn get_api_capabilities(api: drawing_api::GraphicsApi) -> Option<drawing_api::Capabilities> {
        let mut capabilities = Capabilities {
            api: GraphicsApi::OpenGL { major: 3, minor: 1 },
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
                    capabilities.api = GraphicsApi::OpenGL { major: 3, minor: 1 };
                    Some(capabilities)
                } else {
                    None
                }
            }
            drawing_api::GraphicsApi::OpenGLES { major, minor: _ } => {
                if major >= 2 {
                    capabilities.api = GraphicsApi::OpenGLES { major: 2, minor: 0 };
                    Some(capabilities)
                } else {
                    None
                }
            }
            drawing_api::GraphicsApi::Vulkan { major, minor } => {
                if major >= 2 || major == 1 && minor >= 1 {
                    capabilities.api = GraphicsApi::Vulkan { major: 1, minor: 1 };
                    Some(capabilities)
                } else {
                    None
                }
            }
        }
    }

    fn get_capabilities(&self) -> Capabilities {
        Self::get_api_capabilities(GraphicsApi::OpenGLES { major: 2, minor: 0 }).unwrap()
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

    unsafe fn new_color_source_from_fragment_program(
        &self,
        frag_program: &Self::FragmentProgram,
        samplers: &[Self::Texture],
        uniform_data: &[u8],
    ) -> ColorSource<Self::Texture, Self::ColorSourceFragment> {
        let color_source = unsafe {
            self.context
                .borrow()
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
    ) -> drawing_api::ImageFilter<Self::ImageFilterFragment> {
        let image_filter = unsafe {
            self.context
                .borrow()
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
        drawing_api::ImageFilter::Fragment {
            image_filter: image_filter_fragment,
        }
    }
}

impl drawing_api::ContextGl for ImpellerContextGl {
    unsafe fn new_gl<F>(loadfn: F) -> Result<Self, &'static str>
    where
        F: FnMut(&str) -> *mut c_void,
    {
        unsafe {
            let context = Rc::new(RefCell::new(impellers::Context::new_opengl_es(loadfn)?));
            Ok(Self { context })
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
                    impellers::PixelFormat::RGBA8888,
                    impellers::ISize::new(width as i64, height as i64),
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
}
