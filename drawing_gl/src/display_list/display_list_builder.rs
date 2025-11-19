use drawing_api::{DipPoint, PixelLength, PixelPoint};

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

    fn build(self) -> Result<Self::DisplayList, &'static str> {
        Ok(self.display_list)
    }
}
