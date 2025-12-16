use crate::{
    generic::device::{convert_color, Color},
    GlFragmentProgram, GlTexture,
};
use drawing_api::{BlendMode, DrawStyle, StrokeCap, StrokeJoin};

#[derive(Clone)]
pub struct Paint {
    pub(crate) color: Color,
    pub(crate) blend_mode: BlendMode,
    pub(crate) draw_style: DrawStyle,
    pub(crate) stroke_cap: StrokeCap,
    pub(crate) stroke_join: StrokeJoin,
    pub(crate) stroke_width: f32,
    pub(crate) stroke_miter: f32,
    pub(crate) color_filter: Option<drawing_api::ColorFilter>,
    pub(crate) image_filter: Option<drawing_api::ImageFilter<GlTexture, GlFragmentProgram>>,
    pub(crate) color_source: Option<drawing_api::ColorSource<GlTexture, GlFragmentProgram>>,
    pub(crate) mask_filter: Option<drawing_api::MaskFilter>,
}

impl Default for Paint {
    fn default() -> Self {
        Self {
            color: [0.0f32, 0.0f32, 0.0f32, 1.0f32],
            blend_mode: BlendMode::SourceOver,
            draw_style: DrawStyle::Fill,
            stroke_cap: StrokeCap::Butt,
            stroke_join: StrokeJoin::Miter,
            stroke_width: 0.0f32, // hairline width
            stroke_miter: 4.0f32,
            color_filter: None,
            image_filter: None,
            color_source: None,
            mask_filter: None,
        }
    }
}

impl drawing_api::Paint for Paint {
    type FragmentProgram = GlFragmentProgram;
    type Texture = GlTexture;

    fn set_color(&mut self, color: impl Into<drawing_api::Color>) {
        let color = color.into();
        self.color = convert_color(&color);
    }

    fn set_blend_mode(&mut self, blend_mode: BlendMode) {
        self.blend_mode = blend_mode;
    }

    fn set_draw_style(&mut self, draw_style: drawing_api::DrawStyle) {
        self.draw_style = draw_style;
    }

    fn set_stroke_cap(&mut self, cap: StrokeCap) {
        self.stroke_cap = cap;
    }

    fn set_stroke_join(&mut self, join: StrokeJoin) {
        self.stroke_join = join;
    }

    fn set_stroke_width(&mut self, width: f32) {
        self.stroke_width = width;
    }

    fn set_stroke_miter(&mut self, miter: f32) {
        self.stroke_miter = miter;
    }

    fn set_color_source(
        &mut self,
        color_source: Option<drawing_api::ColorSource<Self::Texture, Self::FragmentProgram>>,
    ) {
        self.color_source = color_source;
    }

    fn set_color_filter(&mut self, color_filter: Option<drawing_api::ColorFilter>) {
        self.color_filter = color_filter;
    }

    fn set_image_filter(
        &mut self,
        image_filter: Option<drawing_api::ImageFilter<Self::Texture, Self::FragmentProgram>>,
    ) {
        self.image_filter = image_filter;
    }

    fn set_mask_filter(&mut self, mask_filter: Option<drawing_api::MaskFilter>) {
        self.mask_filter = mask_filter;
    }
}
