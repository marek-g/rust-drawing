use std::borrow::Cow;

use crate::{Capabilities, ColorSource, GraphicsApi, ImageFilter, TextureDescriptor};

/// An abstraction over graphics context (like OpenGL or Vulkan context).
///
/// This object contains common methods for all backends.
pub trait Context: Clone + 'static {
    type ColorSourceFragment: crate::ColorSourceFragment;
    type DisplayList: crate::DisplayList;
    type DisplayListBuilder: crate::DisplayListBuilder<
        DisplayList = Self::DisplayList,
        ImageFilterFragment = Self::ImageFilterFragment,
        Paint = Self::Paint,
        ParagraphBuilder = Self::ParagraphBuilder,
        PathBuilder = Self::PathBuilder,
        Texture = Self::Texture,
    >;
    type Fonts: crate::Fonts;
    type FragmentProgram: crate::FragmentProgram;
    type ImageFilterFragment: crate::ImageFilterFragment;
    type Paint: crate::Paint<
        ColorSourceFragment = Self::ColorSourceFragment,
        ImageFilterFragment = Self::ImageFilterFragment,
        Texture = Self::Texture,
    >;
    type ParagraphBuilder: crate::ParagraphBuilder<
        Texture = Self::Texture,
        Paint = Self::Paint,
        Fonts = Self::Fonts,
    >;
    type PathBuilder: crate::PathBuilder;
    type Surface: crate::Surface<DisplayList = Self::DisplayList>;
    type Texture: crate::Texture;

    /// Gets implementation capabilities depending on graphics API.
    fn get_api_capabilities(api: GraphicsApi) -> Option<Capabilities>;

    /// Gets implementation capabilities of the current instance.
    fn get_capabilities(&self) -> Capabilities;

    /// Creates a new texture.
    unsafe fn create_texture(
        &self,
        contents: Cow<'static, [u8]>,
        descriptor: TextureDescriptor,
    ) -> Result<Self::Texture, &'static str>;

    /// Creates a color source whose pixels are shaded by a fragment program.
    unsafe fn new_color_source_from_fragment_program(
        &self,
        frag_program: &Self::FragmentProgram,
        samplers: &[Self::Texture],
        uniform_data: &[u8],
    ) -> ColorSource<Self::Texture, Self::ColorSourceFragment>;

    /// Creates an image filter where each pixel is shaded by a fragment program.
    unsafe fn new_image_filter_from_fragment_program(
        &self,
        frag_program: &Self::FragmentProgram,
        samplers: &[Self::Texture],
        uniform_data: &[u8],
    ) -> ImageFilter<Self::ImageFilterFragment>;
}
