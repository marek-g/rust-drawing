use crate::{DeviceRect, DipPoint, DipRect, Rect};

use super::TextureSampling;

pub trait DisplayListBuilder {
    type DisplayList;
    type Paint: crate::Paint;
    type Texture: crate::Texture;

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

    /// Draw a portion of texture at the specified location.
    fn draw_texture_rect(
        &mut self,
        texture: &Self::Texture,
        src_rect: impl Into<DeviceRect>,
        dst_rect: impl Into<DipRect>,
        sampling: TextureSampling,
        paint: Option<&Self::Paint>,
    );

    fn build(self) -> Result<Self::DisplayList, &'static str>;
}
