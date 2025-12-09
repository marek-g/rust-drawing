use std::{borrow::Cow, os::raw::c_void};

use crate::{ColorFormat, TextureDescriptor};

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
    type Surface: crate::Surface;
    type Texture: crate::Texture;

    /// Create an OpenGL context.
    fn new_gl_context<F>(loadfn: F) -> Result<Self, &'static str>
    where
        F: FnMut(&str) -> *mut c_void;

    /// Creates a new surface by wrapping an existing OpenGL framebuffer object.
    fn wrap_gl_framebuffer(
        &mut self,
        framebuffer_id: u32,
        width: u32,
        height: u32,
        color_format: ColorFormat,
    ) -> Result<Self::Surface, &'static str>;

    /// Creates a texture with an externally created OpenGL texture handle.
    fn adopt_gl_texture(
        &self,
        texture_handle: u32,
        descriptor: TextureDescriptor,
    ) -> Result<Self::Texture, &'static str>;

    /// Creates a new texture.
    fn create_texture(
        &self,
        contents: Cow<'static, [u8]>,
        descriptor: TextureDescriptor,
    ) -> Result<Self::Texture, &'static str>;

    /// Creates a new fragment shader using compiled program.
    fn create_fragment_shader(
        &self,
        program: Cow<'static, [u8]>,
    ) -> Result<Self::FragmentShader, &'static str>;

    /// Draws a display list on the surface.
    fn draw(
        &mut self,
        surface: &mut Self::Surface,
        display_list: &<Self::DisplayListBuilder as crate::DisplayListBuilder>::DisplayList,
    ) -> Result<(), &'static str>;
}
