use crate::ImpellerTexture;

#[derive(Clone)]
pub struct Paint;

impl Default for Paint {
    fn default() -> Self {
        Self {}
    }
}

impl drawing_api::Paint for Paint {
    type Texture = ImpellerTexture;

    fn set_color(&mut self, color: drawing_api::Color) {
        todo!()
    }

    fn set_blend_mode(&mut self, blend_mode: drawing_api::BlendMode) {
        todo!()
    }

    fn set_draw_style(&mut self, draw_style: drawing_api::DrawStyle) {
        todo!()
    }

    fn set_stroke_cap(&mut self, cap: drawing_api::StrokeCap) {
        todo!()
    }

    fn set_stroke_join(&mut self, join: drawing_api::StrokeJoin) {
        todo!()
    }

    fn set_stroke_width(&mut self, width: f32) {
        todo!()
    }

    fn set_stroke_miter(&mut self, miter: f32) {
        todo!()
    }

    fn set_color_filter(&mut self, color_filter: Option<drawing_api::ColorFilter>) {
        todo!()
    }

    fn set_image_filter(&mut self, image_filter: Option<drawing_api::ImageFilter>) {
        todo!()
    }

    fn set_color_source(&mut self, color_source: Option<drawing_api::ColorSource<Self::Texture>>) {
        todo!()
    }

    fn set_mask_blur_filter(&mut self, mask_filter: Option<drawing_api::MaskFilter>) {
        todo!()
    }
}
