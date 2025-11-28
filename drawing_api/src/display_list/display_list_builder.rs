use crate::{DeviceRect, DipPoint, DipRect};

use super::{ImageFilter, TextureSampling};

pub trait DisplayListBuilder {
    type DisplayList;
    type Paint: crate::Paint;
    type ParagraphBuilder: crate::ParagraphBuilder;
    type PathBuilder: crate::PathBuilder;
    type Texture: crate::Texture;

    /// Create a new display list builder.
    /// An optional cull rectangle may be specified.
    fn new(bounds: impl Into<Option<DipRect>>) -> Self;

    /// Stashes the current transformation and clip state onto a save stack
    /// and creates and creates an offscreen layer
    /// onto which subsequent rendering intent will be directed to.
    fn save_layer(
        &mut self,
        bounds: impl Into<DipRect>,
        paint: Option<&Self::Paint>,
        filter: Option<ImageFilter>,
    );

    /// Pops the last entry pushed onto the save stack.
    fn restore(&mut self);

    /// Fills the current clip with the specified paint.
    fn draw_paint(&mut self, paint: &Self::Paint);

    /// Draws a line segment.
    fn draw_line(
        &mut self,
        from: impl Into<DipPoint>,
        to: impl Into<DipPoint>,
        paint: &Self::Paint,
    );

    /// Draws a rectangle.
    fn draw_rect(&mut self, rect: impl Into<DipRect>, paint: &Self::Paint);

    /// Draws a path.
    fn draw_path(
        &mut self,
        path: &<Self::PathBuilder as crate::PathBuilder>::Path,
        paint: &Self::Paint,
    );

    /// Draw a portion of texture at the specified location.
    fn draw_texture_rect(
        &mut self,
        texture: &Self::Texture,
        src_rect: impl Into<DeviceRect>,
        dst_rect: impl Into<DipRect>,
        sampling: TextureSampling,
        paint: Option<&Self::Paint>,
    );

    /// Draws a paragraph at the specified location.
    fn draw_paragraph(
        &mut self,
        location: impl Into<DipPoint>,
        paragraph: &<Self::ParagraphBuilder as crate::ParagraphBuilder>::Paragraph,
    );

    /// Builds display list.
    fn build(self) -> Result<Self::DisplayList, &'static str>;
}
