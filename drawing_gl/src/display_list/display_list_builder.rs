use drawing_api::{
    DeviceRect, DipPoint, DipRect, PixelLength, PixelPoint, PixelRect, PixelSize, TextureSampling,
};
use euclid::rect;

use crate::{GlContextData, GlTexture};

use super::Primitive;

pub struct DisplayListBuilder {
    display_list: Vec<Primitive<GlTexture, crate::Fonts<GlContextData>>>,
}

impl DisplayListBuilder {
    pub fn new() -> Self {
        Self {
            display_list: Vec::new(),
        }
    }
}

impl drawing_api::DisplayListBuilder for DisplayListBuilder {
    type DisplayList = Vec<Primitive<GlTexture, crate::Fonts<GlContextData>>>;
    type Paint = crate::Paint;
    type Texture = crate::GlTexture;

    fn draw_paint(&mut self, paint: &Self::Paint) {
        // TODO: handle other cases
        self.display_list
            .push(Primitive::Clear { color: paint.color });
    }

    fn draw_line(
        &mut self,
        from: impl Into<DipPoint>,
        to: impl Into<DipPoint>,
        paint: &Self::Paint,
    ) {
        let from = from.into();
        let to = to.into();
        self.display_list.push(Primitive::Line {
            color: paint.color,
            thickness: PixelLength::new(1.0f32),
            // TODO: convert
            start_point: PixelPoint::new(from.x, from.y),
            end_point: PixelPoint::new(to.x, to.y),
        });
    }

    fn draw_rect(&mut self, rect: impl Into<DipRect>, paint: &Self::Paint) {
        let rect = rect.into();
        self.display_list.push(Primitive::Rectangle {
            color: paint.color,
            rect: PixelRect::new(
                PixelPoint::new(rect.origin.x, rect.origin.y),
                PixelSize::new(rect.size.width, rect.size.height),
            ),
        });
    }

    fn draw_texture_rect(
        &mut self,
        texture: &Self::Texture,
        src_rect: impl Into<DeviceRect>,
        dst_rect: impl Into<DipRect>,
        sampling: TextureSampling,
        paint: Option<&Self::Paint>,
    ) {
        let src_rect = src_rect.into();
        let dst_rect = dst_rect.into();
        self.display_list.push(Primitive::Image {
            texture: texture.clone(),
            rect: rect(
                dst_rect.origin.x,
                dst_rect.origin.y,
                dst_rect.size.width,
                dst_rect.size.height,
            ),
            uv: [
                src_rect.origin.x,
                src_rect.origin.y,
                src_rect.origin.x + src_rect.size.width,
                src_rect.origin.y + src_rect.size.height,
            ],
        });
    }

    fn build(self) -> Result<Self::DisplayList, &'static str> {
        Ok(self.display_list)
    }
}
