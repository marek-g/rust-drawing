use std::{any::Any, borrow::Cow, rc::Rc, sync::Arc};

use crate::{
    Capabilities, ColorSource, Context, DisplayListBuilder, FragmentProgram, ImageFilter,
    ParagraphBuilder, PixelRect, TextureDescriptor,
};

use super::{
    ColorSourceFragmentObject, DisplayListBuilderObject, FontsObject, FragmentProgramObject,
    ImageFilterFragmentObject, PaintObject, ParagraphBuilderObject, PathBuilderObject,
    TextureObject,
};

pub trait ContextObject {
    /// Gets implementation capabilities of the current instance.
    fn get_capabilities(&self) -> Capabilities;

    /// Creates fonts object.
    fn create_fonts(&self) -> Box<dyn FontsObject>;

    /// Creates display list builder object.
    fn create_display_list_builder(
        &self,
        bounds: Option<PixelRect>,
    ) -> Box<dyn DisplayListBuilderObject>;

    /// Creates paint object.
    fn create_paint(&self) -> Box<dyn PaintObject>;

    /// Creates paragraph builder object.
    fn create_paragraph_builder(
        &self,
        fonts: &dyn FontsObject,
    ) -> Result<Box<dyn ParagraphBuilderObject>, &'static str>;

    /// Create path builder object
    fn create_path_builder(&self) -> Box<dyn PathBuilderObject>;

    /// Creates a new fragment program.
    unsafe fn create_fragment_program(
        &self,
        program: Cow<'static, [u8]>,
    ) -> Result<Box<dyn FragmentProgramObject>, &'static str>;

    /// Creates a new texture.
    unsafe fn create_texture(
        &self,
        contents: Cow<'static, [u8]>,
        descriptor: TextureDescriptor,
    ) -> Result<Box<dyn TextureObject>, &'static str>;

    /// Creates a color source whose pixels are shaded by a fragment program.
    unsafe fn new_color_source_from_fragment_program(
        &self,
        frag_program: &dyn FragmentProgramObject,
        samplers: &[Rc<dyn TextureObject>],
        uniform_data: &[u8],
    ) -> ColorSource<Arc<dyn TextureObject>, Box<dyn ColorSourceFragmentObject>>;

    /// Creates an image filter where each pixel is shaded by a fragment program.
    unsafe fn new_image_filter_from_fragment_program(
        &self,
        frag_program: &dyn FragmentProgramObject,
        samplers: &[Rc<dyn TextureObject>],
        uniform_data: &[u8],
    ) -> ImageFilter<Box<dyn ImageFilterFragmentObject>>;
}

impl<C: Context> ContextObject for C {
    fn get_capabilities(&self) -> Capabilities {
        self.get_capabilities()
    }

    fn create_fonts(&self) -> Box<dyn FontsObject> {
        Box::new(C::Fonts::default())
    }

    fn create_display_list_builder(
        &self,
        bounds: Option<PixelRect>,
    ) -> Box<dyn DisplayListBuilderObject> {
        Box::new(C::DisplayListBuilder::new(bounds))
    }

    fn create_paint(&self) -> Box<dyn PaintObject> {
        Box::new(C::Paint::default())
    }

    fn create_paragraph_builder(
        &self,
        fonts: &dyn FontsObject,
    ) -> Result<Box<dyn ParagraphBuilderObject>, &'static str> {
        Ok(Box::new(C::ParagraphBuilder::new(
            (fonts as &dyn Any).downcast_ref::<C::Fonts>().unwrap(),
        )?))
    }

    fn create_path_builder(&self) -> Box<dyn PathBuilderObject> {
        Box::new(C::PathBuilder::default())
    }

    unsafe fn create_fragment_program(
        &self,
        program: Cow<'static, [u8]>,
    ) -> Result<Box<dyn FragmentProgramObject>, &'static str> {
        let program = unsafe { C::FragmentProgram::new(program)? };
        Ok(Box::new(program))
    }

    unsafe fn create_texture(
        &self,
        contents: Cow<'static, [u8]>,
        descriptor: TextureDescriptor,
    ) -> Result<Box<dyn TextureObject>, &'static str> {
        unsafe {
            let texture = self.create_texture(contents, descriptor)?;
            Ok(Box::new(texture))
        }
    }

    unsafe fn new_color_source_from_fragment_program(
        &self,
        frag_program: &dyn FragmentProgramObject,
        samplers: &[Rc<dyn TextureObject>],
        uniform_data: &[u8],
    ) -> ColorSource<Arc<dyn TextureObject>, Box<dyn ColorSourceFragmentObject>> {
        let frag_program = (frag_program as &dyn Any)
            .downcast_ref::<C::FragmentProgram>()
            .unwrap();
        let samplers = samplers
            .iter()
            .map(|s| {
                (s as &dyn Any)
                    .downcast_ref::<C::Texture>()
                    .unwrap()
                    .clone()
            })
            .collect::<Vec<_>>();
        let color_source = unsafe {
            self.new_color_source_from_fragment_program(frag_program, &samplers, uniform_data)
        };
        match color_source {
            ColorSource::Fragment { color_source } => ColorSource::Fragment {
                color_source: Box::new(color_source),
            },
            _ => panic!("Expected ColorSource::Fragment"),
        }
    }

    unsafe fn new_image_filter_from_fragment_program(
        &self,
        frag_program: &dyn FragmentProgramObject,
        samplers: &[Rc<dyn TextureObject>],
        uniform_data: &[u8],
    ) -> ImageFilter<Box<dyn ImageFilterFragmentObject>> {
        let frag_program = (frag_program as &dyn Any)
            .downcast_ref::<C::FragmentProgram>()
            .unwrap();
        let samplers = samplers
            .iter()
            .map(|s| {
                (s as &dyn Any)
                    .downcast_ref::<C::Texture>()
                    .unwrap()
                    .clone()
            })
            .collect::<Vec<_>>();
        let image_filter = unsafe {
            self.new_image_filter_from_fragment_program(frag_program, &samplers, uniform_data)
        };
        match image_filter {
            ImageFilter::Fragment { image_filter } => ImageFilter::Fragment {
                image_filter: Box::new(image_filter),
            },
            _ => panic!("Expected ImageFilter::Fragment"),
        }
    }
}
