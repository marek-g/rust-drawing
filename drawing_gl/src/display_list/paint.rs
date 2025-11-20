use crate::generic::device::{convert_color, Color};
use drawing_api::{BlendMode, DrawStyle, StrokeCap, StrokeJoin};

pub struct Paint {
    pub(crate) color: Color,
    pub(crate) blend_mode: BlendMode,
    pub(crate) draw_style: DrawStyle,
    pub(crate) stroke_cap: StrokeCap,
    pub(crate) stroke_join: StrokeJoin,
    pub(crate) stroke_width: f32,
    pub(crate) stroke_miter: f32,
}

impl Paint {
    pub fn new() -> Self {
        Self {
            color: [0.0f32, 0.0f32, 0.0f32, 1.0f32],
            blend_mode: BlendMode::SourceOver,
            draw_style: DrawStyle::Fill,
            stroke_cap: StrokeCap::Butt,
            stroke_join: StrokeJoin::Miter,
            stroke_width: 0.0f32, // hairline width
            stroke_miter: 4.0f32,
        }
    }
}

impl drawing_api::Paint for Paint {
    fn set_color(&mut self, color: drawing_api::Color) {
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
}
