use std::{borrow::Cow, os::raw::c_void};

use drawing_api::TextureDescriptor;
use impellers::{ISize, PixelFormat};

use crate::{ImpellerSurface, ImpellerTexture};

#[derive(Clone)]
pub struct ImpellerContext {
    context: impellers::Context,
}

impl ImpellerContext {
    pub fn new_gl_context<F>(loadfn: F) -> Result<Self, &'static str>
    where
        F: FnMut(&str) -> *mut c_void,
    {
        unsafe {
            let context = impellers::Context::new_opengl_es(loadfn)?;
            Ok(Self { context })
        }
    }
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

    fn wrap_gl_framebuffer(
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

    fn adopt_gl_texture(
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

    fn create_texture(
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

    fn create_fragment_shader(
        &self,
        program: Cow<'static, [u8]>,
    ) -> Result<Self::FragmentShader, &'static str> {
        todo!()
    }

    fn draw(
        &mut self,
        surface: &mut Self::Surface,
        display_list: &<Self::DisplayListBuilder as drawing_api::DisplayListBuilder>::DisplayList,
    ) -> Result<(), &'static str> {
        surface.surface.draw_display_list(display_list)
    }
}
