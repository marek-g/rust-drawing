use crate::Color;

use super::{
    BlendMode, ColorFilter, ColorSource, DrawStyle, ImageFilter, MaskFilter, StrokeCap, StrokeJoin,
};

pub trait Paint {
    type Texture: crate::Texture;

    /// Sets the paint color for stroking or filling.
    fn set_color(&mut self, color: Color);

    /// Sets the paint blend mode.
    fn set_blend_mode(&mut self, blend_mode: BlendMode);

    /// Set the paint draw style.
    fn set_draw_style(&mut self, draw_style: DrawStyle);

    /// Sets how strokes rendered using this paint are capped.
    fn set_stroke_cap(&mut self, cap: StrokeCap);

    /// Sets how strokes rendered using this paint are joined.
    fn set_stroke_join(&mut self, join: StrokeJoin);

    /// Sets the width of the strokes rendered using this paint.
    fn set_stroke_width(&mut self, width: f32);

    /// Sets the miter limit of the strokes rendered using this paint.
    fn set_stroke_miter(&mut self, miter: f32);

    /// Sets the color filter of the paint.
    fn set_color_filter(&mut self, color_filter: Option<ColorFilter>);

    /// Sets the image filter of a paint.
    fn set_image_filter(&mut self, image_filter: Option<ImageFilter>);

    /// Sets the color source of the paint.
    fn set_color_source(&mut self, color_source: Option<ColorSource<Self::Texture>>);

    // Set the mask filter of a paint.
    fn set_mask_blur_filter(&mut self, mask_filter: Option<MaskFilter>);
}
