use drawing_api::{Paint, PixelPoint, PixelThickness, Point, Thickness};

use super::Primitive;

pub struct DisplayListBuilder {
    display_list: Vec<Primitive>,
}

impl DisplayListBuilder {
    pub fn new() -> Self {
        Self {
            display_list: Vec::new(),
        }
    }
}

impl drawing_api::DisplayListBuilder for DisplayListBuilder {
    type DisplayList = Vec<Primitive>;
    type Paint = crate::Paint;

    fn draw_line(&mut self, from: PixelPoint, to: PixelPoint, paint: &Self::Paint) {
        self.display_list.push(Primitive::Line {
            color: paint.color,
            thickness: PixelThickness::new(1.0f32),
            start_point: from,
            end_point: to,
        });
    }

    fn build(self) -> Result<Self::DisplayList, &'static str> {
        Ok(self.display_list)
    }
}
