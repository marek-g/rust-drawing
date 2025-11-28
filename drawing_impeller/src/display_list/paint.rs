use drawing_api::{BlendMode, ColorMatrix, ImageFilter, Matrix, TextureSampling, TileMode};

use crate::ImpellerTexture;

#[derive(Clone)]
pub struct Paint {
    pub(crate) paint: impellers::Paint,
}

impl Default for Paint {
    fn default() -> Self {
        Self {
            paint: impellers::Paint::default(),
        }
    }
}

impl drawing_api::Paint for Paint {
    type Texture = ImpellerTexture;

    fn set_color(&mut self, color: drawing_api::Color) {
        self.paint.set_color(convert_color(&color));
    }

    fn set_blend_mode(&mut self, blend_mode: drawing_api::BlendMode) {
        self.paint.set_blend_mode(convert_blend_mode(blend_mode));
    }

    fn set_draw_style(&mut self, draw_style: drawing_api::DrawStyle) {
        self.paint.set_draw_style(match draw_style {
            drawing_api::DrawStyle::Fill => impellers::DrawStyle::Fill,
            drawing_api::DrawStyle::Stroke => impellers::DrawStyle::Stroke,
            drawing_api::DrawStyle::StrokeAndFill => impellers::DrawStyle::StrokeAndFill,
        });
    }

    fn set_stroke_cap(&mut self, cap: drawing_api::StrokeCap) {
        self.paint.set_stroke_cap(match cap {
            drawing_api::StrokeCap::Butt => impellers::StrokeCap::Butt,
            drawing_api::StrokeCap::Round => impellers::StrokeCap::Round,
            drawing_api::StrokeCap::Square => impellers::StrokeCap::Square,
        });
    }

    fn set_stroke_join(&mut self, join: drawing_api::StrokeJoin) {
        self.paint.set_stroke_join(match join {
            drawing_api::StrokeJoin::Miter => impellers::StrokeJoin::Miter,
            drawing_api::StrokeJoin::Round => impellers::StrokeJoin::Round,
            drawing_api::StrokeJoin::Bevel => impellers::StrokeJoin::Bevel,
        });
    }

    fn set_stroke_width(&mut self, width: f32) {
        self.paint.set_stroke_width(width);
    }

    fn set_stroke_miter(&mut self, miter: f32) {
        self.paint.set_stroke_miter(miter);
    }

    fn set_color_filter(&mut self, color_filter: Option<drawing_api::ColorFilter>) {
        if let Some(color_filter) = color_filter {
            let color_filter = match color_filter {
                drawing_api::ColorFilter::Blend(color, blend_mode) => {
                    impellers::ColorFilter::new_blend(
                        convert_color(&color),
                        convert_blend_mode(blend_mode),
                    )
                }
                drawing_api::ColorFilter::Matrix(color_matrix) => {
                    impellers::ColorFilter::new_matrix(convert_color_matrix(color_matrix))
                }
            };
            self.paint.set_color_filter(&color_filter);
        } else {
            todo!("Clearing color filter is not implemented")
        }
    }

    fn set_image_filter(&mut self, image_filter: Option<drawing_api::ImageFilter>) {
        if let Some(image_filter) = image_filter {
            let image_filter = convert_image_filter(image_filter);
            self.paint.set_image_filter(&image_filter);
        } else {
            todo!("Clearing image filter is not implemented")
        }
    }

    fn set_color_source(&mut self, color_source: Option<drawing_api::ColorSource<Self::Texture>>) {
        if let Some(color_source) = color_source {
            let color_source = match color_source {
                drawing_api::ColorSource::LinearGradient {
                    start,
                    end,
                    colors,
                    stops,
                    tile_mode,
                    transformation,
                } => impellers::ColorSource::new_linear_gradient(
                    start,
                    end,
                    &colors
                        .into_iter()
                        .map(|c| convert_color(&c))
                        .collect::<Vec<_>>(),
                    &stops,
                    convert_tile_mode(tile_mode),
                    transformation.map(|t| convert_matrix(t)).as_ref(),
                ),
                drawing_api::ColorSource::RadialGradient {
                    center,
                    radius,
                    colors,
                    stops,
                    tile_mode,
                    transformation,
                } => impellers::ColorSource::new_radial_gradient(
                    center,
                    radius,
                    &colors
                        .into_iter()
                        .map(|c| convert_color(&c))
                        .collect::<Vec<_>>(),
                    &stops,
                    convert_tile_mode(tile_mode),
                    transformation.map(|t| convert_matrix(t)).as_ref(),
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
                    start_center,
                    start_radius,
                    end_center,
                    end_radius,
                    &colors
                        .into_iter()
                        .map(|c| convert_color(&c))
                        .collect::<Vec<_>>(),
                    &stops,
                    convert_tile_mode(tile_mode),
                    transformation.map(|t| convert_matrix(t)).as_ref(),
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
                    center,
                    start,
                    end,
                    &colors
                        .into_iter()
                        .map(|c| convert_color(&c))
                        .collect::<Vec<_>>(),
                    &stops,
                    convert_tile_mode(tile_mode),
                    transformation.map(|t| convert_matrix(t)).as_ref(),
                ),
                drawing_api::ColorSource::Image {
                    image,
                    horizontal_tile_mode,
                    vertical_tile_mode,
                    sampling,
                    transformation,
                } => todo!("Add support for sys:Matrix"), /*impellers::ColorSource::new_image(
                                                              &image.texture,
                                                              convert_tile_mode(horizontal_tile_mode),
                                                              convert_tile_mode(vertical_tile_mode),
                                                              convert_texture_sampling(sampling),
                                                              //impellers::DisplayListBuilder::get_transform(&self), //transformation.map(|t| convert_matrix(t)).as_ref(),
                                                          ),*/
            };
            self.paint.set_color_source(&color_source);
        } else {
            todo!("Clearing color source is not implemented")
        }
    }

    fn set_mask_blur_filter(&mut self, mask_filter: Option<drawing_api::MaskFilter>) {
        if let Some(mask_filter) = mask_filter {
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
            self.paint.set_mask_filter(&mask_filter);
        } else {
            todo!("Clearing mask blur filter is not implemented")
        }
    }
}

pub fn convert_color(color: &drawing_api::Color) -> impellers::Color {
    impellers::Color::new_srgba(color.red, color.green, color.blue, color.alpha)
}

pub fn convert_blend_mode(blend_mode: BlendMode) -> impellers::BlendMode {
    match blend_mode {
        drawing_api::BlendMode::Clear => impellers::BlendMode::Clear,
        drawing_api::BlendMode::Source => impellers::BlendMode::Source,
        drawing_api::BlendMode::Destination => impellers::BlendMode::Destination,
        drawing_api::BlendMode::SourceOver => impellers::BlendMode::SourceOver,
        drawing_api::BlendMode::DestinationOver => impellers::BlendMode::DestinationOver,
        drawing_api::BlendMode::SourceIn => impellers::BlendMode::SourceIn,
        drawing_api::BlendMode::DestinationIn => impellers::BlendMode::DestinationIn,
        drawing_api::BlendMode::SourceOut => impellers::BlendMode::SourceOut,
        drawing_api::BlendMode::DestinationOut => impellers::BlendMode::DestinationOut,
        drawing_api::BlendMode::SourceATop => impellers::BlendMode::SourceATop,
        drawing_api::BlendMode::DestinationATop => impellers::BlendMode::DestinationATop,
        drawing_api::BlendMode::Xor => impellers::BlendMode::Xor,
        drawing_api::BlendMode::Plus => impellers::BlendMode::Plus,
        drawing_api::BlendMode::Modulate => impellers::BlendMode::Modulate,
        drawing_api::BlendMode::Screen => impellers::BlendMode::Screen,
        drawing_api::BlendMode::Overlay => impellers::BlendMode::Overlay,
        drawing_api::BlendMode::Darken => impellers::BlendMode::Darken,
        drawing_api::BlendMode::Lighten => impellers::BlendMode::Lighten,
        drawing_api::BlendMode::ColorDodge => impellers::BlendMode::ColorDodge,
        drawing_api::BlendMode::ColorBurn => impellers::BlendMode::ColorBurn,
        drawing_api::BlendMode::HardLight => impellers::BlendMode::HardLight,
        drawing_api::BlendMode::SoftLight => impellers::BlendMode::SoftLight,
        drawing_api::BlendMode::Difference => impellers::BlendMode::Difference,
        drawing_api::BlendMode::Exclusion => impellers::BlendMode::Exclusion,
        drawing_api::BlendMode::Multiply => impellers::BlendMode::Multiply,
        drawing_api::BlendMode::Hue => impellers::BlendMode::Hue,
        drawing_api::BlendMode::Saturation => impellers::BlendMode::Saturation,
        drawing_api::BlendMode::Color => impellers::BlendMode::Color,
        drawing_api::BlendMode::Luminosity => impellers::BlendMode::Luminosity,
    }
}

pub fn convert_tile_mode(tile_mode: TileMode) -> impellers::TileMode {
    match tile_mode {
        drawing_api::TileMode::Clamp => impellers::TileMode::Clamp,
        drawing_api::TileMode::Repeat => impellers::TileMode::Repeat,
        drawing_api::TileMode::Mirror => impellers::TileMode::Mirror,
        drawing_api::TileMode::Decal => impellers::TileMode::Decal,
    }
}

pub fn convert_texture_sampling(texture_sampling: TextureSampling) -> impellers::TextureSampling {
    match texture_sampling {
        TextureSampling::NearestNeighbor => impellers::TextureSampling::NearestNeighbor,
        TextureSampling::Linear => impellers::TextureSampling::Linear,
    }
}

pub fn convert_image_filter(image_filter: ImageFilter) -> impellers::ImageFilter {
    match image_filter {
        drawing_api::ImageFilter::Blur {
            x_sigma,
            y_sigma,
            tile_mode,
        } => impellers::ImageFilter::new_blur(x_sigma, y_sigma, convert_tile_mode(tile_mode)),
        drawing_api::ImageFilter::Dilate { x_radius, y_radius } => {
            impellers::ImageFilter::new_dilate(x_radius, y_radius)
        }
        drawing_api::ImageFilter::Erode { x_radius, y_radius } => {
            impellers::ImageFilter::new_erode(x_radius, y_radius)
        }
        drawing_api::ImageFilter::Matrix { matrix, sampling } => {
            impellers::ImageFilter::new_matrix(
                &convert_matrix(matrix),
                match sampling {
                    drawing_api::TextureSampling::NearestNeighbor => {
                        impellers::TextureSampling::NearestNeighbor
                    }
                    drawing_api::TextureSampling::Linear => impellers::TextureSampling::Linear,
                },
            )
        }
        drawing_api::ImageFilter::Compose { outer, inner } => {
            let outer = convert_image_filter(*outer);
            let inner = convert_image_filter(*inner);
            impellers::ImageFilter::new_compose(&outer, &inner)
        }
    }
}

pub fn convert_color_matrix(color_matrix: ColorMatrix) -> impellers::ColorMatrix {
    impellers::ColorMatrix { m: color_matrix.m }
}

pub fn convert_matrix(matrix: Matrix) -> impellers::Matrix {
    impellers::Matrix::from_array(matrix.to_array())
}
