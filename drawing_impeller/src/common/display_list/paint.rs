use std::sync::{Arc, Mutex};

use crate::ImpellerTexture;

use super::{
    convert_blend_mode, convert_color, convert_color_matrix, convert_image_filter, convert_matrix,
    convert_point, convert_texture_sampling, convert_tile_mode, ColorSourceFragment,
    ImageFilterFragment,
};

#[derive(Clone)]
pub struct Paint {
    pub(crate) paint: Arc<Mutex<impellers::Paint>>,
}

impl Default for Paint {
    fn default() -> Self {
        Self {
            paint: Arc::new(Mutex::new(impellers::Paint::default())),
        }
    }
}

impl drawing_api::Paint for Paint {
    type ColorSourceFragment = crate::ColorSourceFragment;
    type ImageFilterFragment = crate::ImageFilterFragment;
    type Texture = ImpellerTexture;

    fn set_color(&mut self, color: impl Into<drawing_api::Color>) {
        let color = color.into();
        self.paint.lock().unwrap().set_color(convert_color(&color));
    }

    fn set_blend_mode(&mut self, blend_mode: drawing_api::BlendMode) {
        self.paint
            .lock()
            .unwrap()
            .set_blend_mode(convert_blend_mode(blend_mode));
    }

    fn set_draw_style(&mut self, draw_style: drawing_api::DrawStyle) {
        self.paint.lock().unwrap().set_draw_style(match draw_style {
            drawing_api::DrawStyle::Fill => impellers::DrawStyle::Fill,
            drawing_api::DrawStyle::Stroke => impellers::DrawStyle::Stroke,
            drawing_api::DrawStyle::StrokeAndFill => impellers::DrawStyle::StrokeAndFill,
        });
    }

    fn set_stroke_cap(&mut self, cap: drawing_api::StrokeCap) {
        self.paint.lock().unwrap().set_stroke_cap(match cap {
            drawing_api::StrokeCap::Butt => impellers::StrokeCap::Butt,
            drawing_api::StrokeCap::Round => impellers::StrokeCap::Round,
            drawing_api::StrokeCap::Square => impellers::StrokeCap::Square,
        });
    }

    fn set_stroke_join(&mut self, join: drawing_api::StrokeJoin) {
        self.paint.lock().unwrap().set_stroke_join(match join {
            drawing_api::StrokeJoin::Miter => impellers::StrokeJoin::Miter,
            drawing_api::StrokeJoin::Round => impellers::StrokeJoin::Round,
            drawing_api::StrokeJoin::Bevel => impellers::StrokeJoin::Bevel,
        });
    }

    fn set_stroke_width(&mut self, width: f32) {
        self.paint.lock().unwrap().set_stroke_width(width);
    }

    fn set_stroke_miter(&mut self, miter: f32) {
        self.paint.lock().unwrap().set_stroke_miter(miter);
    }

    fn set_color_source(
        &mut self,
        color_source: drawing_api::ColorSource<Self::Texture, ColorSourceFragment>,
    ) {
        let color_source = match color_source {
            drawing_api::ColorSource::LinearGradient {
                start,
                end,
                colors,
                stops,
                tile_mode,
                transformation,
            } => impellers::ColorSource::new_linear_gradient(
                convert_point(&start),
                convert_point(&end),
                &colors
                    .into_iter()
                    .map(|c| convert_color(&c))
                    .collect::<Vec<_>>(),
                &stops,
                convert_tile_mode(tile_mode),
                transformation.map(|t| convert_matrix(&t)).as_ref(),
            ),
            drawing_api::ColorSource::RadialGradient {
                center,
                radius,
                colors,
                stops,
                tile_mode,
                transformation,
            } => impellers::ColorSource::new_radial_gradient(
                convert_point(&center),
                radius,
                &colors
                    .into_iter()
                    .map(|c| convert_color(&c))
                    .collect::<Vec<_>>(),
                &stops,
                convert_tile_mode(tile_mode),
                transformation.map(|t| convert_matrix(&t)).as_ref(),
            ),
            drawing_api::ColorSource::ConicalGradient {
                start_center,
                start_radius,
                end_center,
                end_radius,
                colors,
                stops,
                tile_mode,
                transformation,
            } => impellers::ColorSource::new_conical_gradient(
                convert_point(&start_center),
                start_radius,
                convert_point(&end_center),
                end_radius,
                &colors
                    .into_iter()
                    .map(|c| convert_color(&c))
                    .collect::<Vec<_>>(),
                &stops,
                convert_tile_mode(tile_mode),
                transformation.map(|t| convert_matrix(&t)).as_ref(),
            ),
            drawing_api::ColorSource::SweepGradient {
                center,
                start,
                end,
                colors,
                stops,
                tile_mode,
                transformation,
            } => impellers::ColorSource::new_sweep_gradient(
                convert_point(&center),
                start,
                end,
                &colors
                    .into_iter()
                    .map(|c| convert_color(&c))
                    .collect::<Vec<_>>(),
                &stops,
                convert_tile_mode(tile_mode),
                transformation.map(|t| convert_matrix(&t)).as_ref(),
            ),
            drawing_api::ColorSource::Image {
                image,
                horizontal_tile_mode,
                vertical_tile_mode,
                sampling,
                transformation,
            } => impellers::ColorSource::new_image(
                &image.texture,
                convert_tile_mode(horizontal_tile_mode),
                convert_tile_mode(vertical_tile_mode),
                convert_texture_sampling(sampling),
                transformation.map(|t| convert_matrix(&t)).as_ref(),
            ),
            drawing_api::ColorSource::Fragment { color_source } => color_source.color_source,
        };
        self.paint.lock().unwrap().set_color_source(&color_source);
    }

    fn set_color_filter(&mut self, color_filter: drawing_api::ColorFilter) {
        let color_filter = match color_filter {
            drawing_api::ColorFilter::Blend(color, blend_mode) => {
                impellers::ColorFilter::new_blend(
                    convert_color(&color),
                    convert_blend_mode(blend_mode),
                )
            }
            drawing_api::ColorFilter::Matrix(color_matrix) => {
                impellers::ColorFilter::new_matrix(convert_color_matrix(&color_matrix))
            }
        };
        self.paint.lock().unwrap().set_color_filter(&color_filter);
    }

    fn set_image_filter(&mut self, image_filter: drawing_api::ImageFilter<ImageFilterFragment>) {
        let image_filter = convert_image_filter(image_filter);
        self.paint.lock().unwrap().set_image_filter(&image_filter);
    }

    fn set_mask_filter(&mut self, mask_filter: drawing_api::MaskFilter) {
        let mask_filter = match mask_filter {
            drawing_api::MaskFilter::Blur { style, sigma } => impellers::MaskFilter::new_blur(
                match style {
                    drawing_api::BlurStyle::Normal => impellers::BlurStyle::Normal,
                    drawing_api::BlurStyle::Solid => impellers::BlurStyle::Solid,
                    drawing_api::BlurStyle::Outer => impellers::BlurStyle::Outer,
                    drawing_api::BlurStyle::Inner => impellers::BlurStyle::Inner,
                },
                sigma,
            ),
        };
        self.paint.lock().unwrap().set_mask_filter(&mask_filter);
    }
}
