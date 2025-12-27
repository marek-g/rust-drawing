use std::{any::Any, sync::Arc};

use crate::{
    BlendMode, Color, ColorFilter, ColorSource, ColorSourceFragment, DrawStyle, ImageFilter,
    ImageFilterFragment, MaskFilter, Paint, StrokeCap, StrokeJoin, Texture,
};

use super::{ColorSourceFragmentObject, ImageFilterFragmentObject, TextureObject};

pub trait PaintObject: Any {
    /// Sets the paint color for stroking or filling.
    fn set_color(&mut self, color: Color);

    fn with_color(self: Box<Self>, color: Color) -> Box<dyn PaintObject>;

    /// Sets the paint blend mode.
    fn set_blend_mode(&mut self, blend_mode: BlendMode);

    fn with_blend_mode(self: Box<Self>, blend_mode: BlendMode) -> Box<dyn PaintObject>;

    /// Set the paint draw style.
    fn set_draw_style(&mut self, draw_style: DrawStyle);

    fn with_draw_style(self: Box<Self>, draw_style: DrawStyle) -> Box<dyn PaintObject>;

    /// Sets how strokes rendered using this paint are capped.
    fn set_stroke_cap(&mut self, cap: StrokeCap);

    fn with_stroke_cap(self: Box<Self>, cap: StrokeCap) -> Box<dyn PaintObject>;

    /// Sets how strokes rendered using this paint are joined.
    fn set_stroke_join(&mut self, join: StrokeJoin);

    fn with_stroke_join(self: Box<Self>, join: StrokeJoin) -> Box<dyn PaintObject>;

    /// Sets the width of the strokes rendered using this paint.
    fn set_stroke_width(&mut self, width: f32);

    fn with_stroke_width(self: Box<Self>, width: f32) -> Box<dyn PaintObject>;

    /// Sets the miter limit of the strokes rendered using this paint.
    fn set_stroke_miter(&mut self, miter: f32);

    fn with_stroke_miter(self: Box<Self>, miter: f32) -> Box<dyn PaintObject>;

    /// Sets the color source of the paint.
    fn set_color_source(
        &mut self,
        color_source: ColorSource<Arc<dyn TextureObject>, Box<dyn ColorSourceFragmentObject>>,
    );

    fn with_color_source(
        self: Box<Self>,
        color_source: ColorSource<Arc<dyn TextureObject>, Box<dyn ColorSourceFragmentObject>>,
    ) -> Box<dyn PaintObject>;

    /// Sets the color filter of the paint.
    fn set_color_filter(&mut self, color_filter: ColorFilter);

    fn with_color_filter(self: Box<Self>, color_filter: ColorFilter) -> Box<dyn PaintObject>;

    /// Sets the image filter of a paint.
    ///
    /// Image filters are functions that are applied to regions of a texture to produce a single color.
    fn set_image_filter(&mut self, image_filter: ImageFilter<Box<dyn ImageFilterFragmentObject>>);

    fn with_image_filter(
        self: Box<Self>,
        image_filter: ImageFilter<Box<dyn ImageFilterFragmentObject>>,
    ) -> Box<dyn PaintObject>;

    /// Sets the mask filter of a paint.
    ///
    /// Mask filters are functions that are applied over a shape after it has been drawn but before it has been blended into the final image.
    fn set_mask_filter(&mut self, mask_filter: MaskFilter);

    fn with_mask_filter(self: Box<Self>, mask_filter: MaskFilter) -> Box<dyn PaintObject>;
}

impl<P: Paint> PaintObject for P {
    fn set_color(&mut self, color: Color) {
        self.set_color(color);
    }

    fn with_color(mut self: Box<Self>, color: Color) -> Box<dyn PaintObject> {
        self.set_color(color);
        self
    }

    fn set_blend_mode(&mut self, blend_mode: BlendMode) {
        self.set_blend_mode(blend_mode);
    }

    fn with_blend_mode(mut self: Box<Self>, blend_mode: BlendMode) -> Box<dyn PaintObject> {
        self.set_blend_mode(blend_mode);
        self
    }

    fn set_draw_style(&mut self, draw_style: DrawStyle) {
        self.set_draw_style(draw_style);
    }

    fn with_draw_style(mut self: Box<Self>, draw_style: DrawStyle) -> Box<dyn PaintObject> {
        self.set_draw_style(draw_style);
        self
    }

    fn set_stroke_cap(&mut self, cap: StrokeCap) {
        self.set_stroke_cap(cap);
    }

    fn with_stroke_cap(mut self: Box<Self>, cap: StrokeCap) -> Box<dyn PaintObject> {
        self.set_stroke_cap(cap);
        self
    }

    fn set_stroke_join(&mut self, join: StrokeJoin) {
        self.set_stroke_join(join);
    }

    fn with_stroke_join(mut self: Box<Self>, join: StrokeJoin) -> Box<dyn PaintObject> {
        self.set_stroke_join(join);
        self
    }

    fn set_stroke_width(&mut self, width: f32) {
        self.set_stroke_width(width);
    }

    fn with_stroke_width(mut self: Box<Self>, width: f32) -> Box<dyn PaintObject> {
        self.set_stroke_width(width);
        self
    }

    fn set_stroke_miter(&mut self, miter: f32) {
        self.set_stroke_miter(miter);
    }

    fn with_stroke_miter(mut self: Box<Self>, miter: f32) -> Box<dyn PaintObject> {
        self.set_stroke_miter(miter);
        self
    }

    fn set_color_source(
        &mut self,
        color_source: ColorSource<Arc<dyn TextureObject>, Box<dyn ColorSourceFragmentObject>>,
    ) {
        let color_source = convert_color_source::<P::Texture, P::ColorSourceFragment>(color_source);
        self.set_color_source(color_source);
    }

    fn with_color_source(
        mut self: Box<Self>,
        color_source: ColorSource<Arc<dyn TextureObject>, Box<dyn ColorSourceFragmentObject>>,
    ) -> Box<dyn PaintObject> {
        let color_source = convert_color_source::<P::Texture, P::ColorSourceFragment>(color_source);
        self.set_color_source(color_source);
        self
    }

    fn set_color_filter(&mut self, color_filter: ColorFilter) {
        self.set_color_filter(color_filter);
    }

    fn with_color_filter(mut self: Box<Self>, color_filter: ColorFilter) -> Box<dyn PaintObject> {
        self.set_color_filter(color_filter);
        self
    }

    fn set_image_filter(&mut self, image_filter: ImageFilter<Box<dyn ImageFilterFragmentObject>>) {
        let image_filter = convert_image_filter::<P::ImageFilterFragment>(image_filter);
        self.set_image_filter(image_filter);
    }

    fn with_image_filter(
        mut self: Box<Self>,
        image_filter: ImageFilter<Box<dyn ImageFilterFragmentObject>>,
    ) -> Box<dyn PaintObject> {
        let image_filter = convert_image_filter::<P::ImageFilterFragment>(image_filter);
        self.set_image_filter(image_filter);
        self
    }

    fn set_mask_filter(&mut self, mask_filter: MaskFilter) {
        self.set_mask_filter(mask_filter);
    }

    fn with_mask_filter(mut self: Box<Self>, mask_filter: MaskFilter) -> Box<dyn PaintObject> {
        self.set_mask_filter(mask_filter);
        self
    }
}

fn convert_color_source<T: Texture, F: ColorSourceFragment>(
    color_source: ColorSource<Arc<dyn TextureObject>, Box<dyn ColorSourceFragmentObject>>,
) -> ColorSource<T, F> {
    match color_source {
        ColorSource::LinearGradient {
            start,
            end,
            colors,
            stops,
            tile_mode,
            transformation,
        } => ColorSource::LinearGradient {
            start,
            end,
            colors,
            stops,
            tile_mode,
            transformation,
        },
        ColorSource::RadialGradient {
            center,
            radius,
            colors,
            stops,
            tile_mode,
            transformation,
        } => ColorSource::RadialGradient {
            center,
            radius,
            colors,
            stops,
            tile_mode,
            transformation,
        },
        ColorSource::ConicalGradient {
            start_center,
            start_radius,
            end_center,
            end_radius,
            colors,
            stops,
            tile_mode,
            transformation,
        } => ColorSource::ConicalGradient {
            start_center,
            start_radius,
            end_center,
            end_radius,
            colors,
            stops,
            tile_mode,
            transformation,
        },
        ColorSource::SweepGradient {
            center,
            start,
            end,
            colors,
            stops,
            tile_mode,
            transformation,
        } => ColorSource::SweepGradient {
            center,
            start,
            end,
            colors,
            stops,
            tile_mode,
            transformation,
        },
        ColorSource::Image {
            image,
            horizontal_tile_mode,
            vertical_tile_mode,
            sampling,
            transformation,
        } => {
            let image = (&image as &dyn Any).downcast_ref::<T>().unwrap().clone();
            ColorSource::Image {
                image,
                horizontal_tile_mode,
                vertical_tile_mode,
                sampling,
                transformation,
            }
        }
        ColorSource::Fragment { color_source } => {
            let color_source = *(color_source as Box<dyn Any>).downcast::<F>().unwrap();
            ColorSource::Fragment { color_source }
        }
    }
}

pub(crate) fn convert_image_filter<F: ImageFilterFragment>(
    image_filter: ImageFilter<Box<dyn ImageFilterFragmentObject>>,
) -> ImageFilter<F> {
    match image_filter {
        ImageFilter::Blur {
            x_sigma,
            y_sigma,
            tile_mode,
        } => ImageFilter::Blur {
            x_sigma,
            y_sigma,
            tile_mode,
        },
        ImageFilter::Dilate { x_radius, y_radius } => ImageFilter::Dilate { x_radius, y_radius },
        ImageFilter::Erode { x_radius, y_radius } => ImageFilter::Erode { x_radius, y_radius },
        ImageFilter::Matrix { matrix, sampling } => ImageFilter::Matrix { matrix, sampling },
        ImageFilter::Fragment { image_filter } => {
            let image_filter = *(image_filter as Box<dyn Any>).downcast::<F>().unwrap();
            ImageFilter::Fragment { image_filter }
        }
        ImageFilter::Compose { outer, inner } => {
            let outer = Box::new(convert_image_filter(*outer));
            let inner = Box::new(convert_image_filter(*inner));
            ImageFilter::Compose { outer, inner }
        }
    }
}
