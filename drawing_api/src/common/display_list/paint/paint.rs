use crate::Color;

use super::{
    BlendMode, ColorFilter, ColorSource, DrawStyle, ImageFilter, MaskFilter, StrokeCap, StrokeJoin,
};

pub trait Paint: Default {
    type ColorSourceFragment: crate::ColorSourceFragment;
    type ImageFilterFragment: crate::ImageFilterFragment;
    type Texture: crate::Texture;

    fn color(color: impl Into<Color>) -> Self {
        Self::default().with_color(color)
    }

    fn stroke_color(color: impl Into<Color>, stroke_width: f32) -> Self {
        Self::default()
            .with_color(color)
            .with_draw_style(DrawStyle::Stroke)
            .with_stroke_width(stroke_width)
    }

    fn color_source(color_source: ColorSource<Self::Texture, Self::ColorSourceFragment>) -> Self {
        Self::default().with_color_source(Some(color_source))
    }

    /// Sets the paint color for stroking or filling.
    fn set_color(&mut self, color: impl Into<Color>);

    fn with_color(mut self, color: impl Into<Color>) -> Self {
        self.set_color(color);
        self
    }

    /// Sets the paint blend mode.
    fn set_blend_mode(&mut self, blend_mode: BlendMode);

    fn with_blend_mode(mut self, blend_mode: BlendMode) -> Self {
        self.set_blend_mode(blend_mode);
        self
    }

    /// Set the paint draw style.
    fn set_draw_style(&mut self, draw_style: DrawStyle);

    fn with_draw_style(mut self, draw_style: DrawStyle) -> Self {
        self.set_draw_style(draw_style);
        self
    }

    /// Sets how strokes rendered using this paint are capped.
    fn set_stroke_cap(&mut self, cap: StrokeCap);

    fn with_stroke_cap(mut self, cap: StrokeCap) -> Self {
        self.set_stroke_cap(cap);
        self
    }

    /// Sets how strokes rendered using this paint are joined.
    fn set_stroke_join(&mut self, join: StrokeJoin);

    fn with_stroke_join(mut self, join: StrokeJoin) -> Self {
        self.set_stroke_join(join);
        self
    }

    /// Sets the width of the strokes rendered using this paint.
    fn set_stroke_width(&mut self, width: f32);

    fn with_stroke_width(mut self, width: f32) -> Self {
        self.set_stroke_width(width);
        self
    }

    /// Sets the miter limit of the strokes rendered using this paint.
    fn set_stroke_miter(&mut self, miter: f32);

    fn with_stroke_miter(mut self, miter: f32) -> Self {
        self.set_stroke_miter(miter);
        self
    }

    /// Sets the color source of the paint.
    fn set_color_source(
        &mut self,
        color_source: Option<ColorSource<Self::Texture, Self::ColorSourceFragment>>,
    );

    fn with_color_source(
        mut self,
        color_source: Option<ColorSource<Self::Texture, Self::ColorSourceFragment>>,
    ) -> Self {
        self.set_color_source(color_source);
        self
    }

    /// Sets the color filter of the paint.
    fn set_color_filter(&mut self, color_filter: Option<ColorFilter>);

    fn with_color_filter(mut self, color_filter: Option<ColorFilter>) -> Self {
        self.set_color_filter(color_filter);
        self
    }

    /// Sets the image filter of a paint.
    ///
    /// Image filters are functions that are applied to regions of a texture to produce a single color.
    fn set_image_filter(&mut self, image_filter: Option<ImageFilter<Self::ImageFilterFragment>>);

    fn with_image_filter(
        mut self,
        image_filter: Option<ImageFilter<Self::ImageFilterFragment>>,
    ) -> Self {
        self.set_image_filter(image_filter);
        self
    }

    /// Sets the mask filter of a paint.
    ///
    /// Mask filters are functions that are applied over a shape after it has been drawn but before it has been blended into the final image.
    fn set_mask_filter(&mut self, mask_filter: Option<MaskFilter>);

    fn with_mask_filter(mut self, mask_filter: Option<MaskFilter>) -> Self {
        self.set_mask_filter(mask_filter);
        self
    }
}
