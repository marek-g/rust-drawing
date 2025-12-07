use crate::{ImpellerFragmentShader, ImpellerTexture};

pub fn convert_point(point: &drawing_api::DipPoint) -> impellers::Point {
    impellers::Point::new(point.x, point.y)
}

pub fn convert_to_point(point: &impellers::Point) -> drawing_api::DipPoint {
    drawing_api::DipPoint::new(point.x, point.y)
}

pub fn convert_size(size: &drawing_api::DipSize) -> impellers::Size {
    impellers::Size::new(size.width, size.height)
}

pub fn convert_to_size(size: &impellers::Size) -> drawing_api::DipSize {
    drawing_api::DipSize::new(size.width, size.height)
}

pub fn convert_rect(rect: &drawing_api::DipRect) -> impellers::Rect {
    impellers::Rect::new(convert_point(&rect.origin), convert_size(&rect.size))
}

pub fn convert_to_rect(rect: &impellers::Rect) -> drawing_api::DipRect {
    drawing_api::DipRect::new(convert_to_point(&rect.origin), convert_to_size(&rect.size))
}

pub fn convert_radii(rect: &drawing_api::RoundingRadii) -> impellers::RoundingRadii {
    impellers::RoundingRadii {
        top_left: bytemuck::cast(impellers::Point::new(
            rect.top_left.width,
            rect.top_left.height,
        )),
        bottom_left: bytemuck::cast(impellers::Point::new(
            rect.bottom_left.width,
            rect.bottom_left.height,
        )),
        top_right: bytemuck::cast(impellers::Point::new(
            rect.top_right.width,
            rect.top_right.height,
        )),
        bottom_right: bytemuck::cast(impellers::Point::new(
            rect.bottom_right.width,
            rect.bottom_right.height,
        )),
    }
}

pub fn convert_device_point(point: &drawing_api::DevicePoint) -> impellers::Point {
    impellers::Point::new(point.x, point.y)
}

pub fn convert_device_size(size: &drawing_api::DeviceSize) -> impellers::Size {
    impellers::Size::new(size.width, size.height)
}

pub fn convert_device_rect(rect: &drawing_api::DeviceRect) -> impellers::Rect {
    impellers::Rect::new(
        convert_device_point(&rect.origin),
        convert_device_size(&rect.size),
    )
}

pub fn convert_clip_operation(operation: &drawing_api::ClipOperation) -> impellers::ClipOperation {
    match operation {
        drawing_api::ClipOperation::Difference => impellers::ClipOperation::Difference,
        drawing_api::ClipOperation::Intersect => impellers::ClipOperation::Intersect,
    }
}

pub fn convert_color(color: &drawing_api::Color) -> impellers::Color {
    impellers::Color::new_srgba(color.red, color.green, color.blue, color.alpha)
}

pub fn convert_blend_mode(blend_mode: drawing_api::BlendMode) -> impellers::BlendMode {
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

pub fn convert_tile_mode(tile_mode: drawing_api::TileMode) -> impellers::TileMode {
    match tile_mode {
        drawing_api::TileMode::Clamp => impellers::TileMode::Clamp,
        drawing_api::TileMode::Repeat => impellers::TileMode::Repeat,
        drawing_api::TileMode::Mirror => impellers::TileMode::Mirror,
        drawing_api::TileMode::Decal => impellers::TileMode::Decal,
    }
}

pub fn convert_texture_sampling(
    texture_sampling: drawing_api::TextureSampling,
) -> impellers::TextureSampling {
    match texture_sampling {
        drawing_api::TextureSampling::NearestNeighbor => {
            impellers::TextureSampling::NearestNeighbor
        }
        drawing_api::TextureSampling::Linear => impellers::TextureSampling::Linear,
    }
}

pub fn convert_image_filter(
    image_filter: drawing_api::ImageFilter<ImpellerTexture, ImpellerFragmentShader>,
) -> impellers::ImageFilter {
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
                &convert_matrix(&matrix),
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
        drawing_api::ImageFilter::FragmentShader {
            program,
            samplers,
            data,
        } => todo!(),
    }
}

pub fn convert_fill_type(fill_type: drawing_api::FillType) -> impellers::FillType {
    match fill_type {
        drawing_api::FillType::NonZero => impellers::FillType::NonZero,
        drawing_api::FillType::Odd => impellers::FillType::Odd,
    }
}

pub fn convert_paragraph_style(
    style: &drawing_api::ParagraphStyle<crate::ImpellerTexture, crate::Paint>,
) -> impellers::ParagraphStyle {
    let mut result = impellers::ParagraphStyle::default();
    if let Some(foreground) = &style.foreground {
        result.set_foreground(&foreground.paint);
    }
    if let Some(background) = &style.background {
        result.set_background(&background.paint);
    }
    result.set_font_weight(convert_font_weight(style.weight));
    result.set_font_style(convert_font_style(style.style));
    result.set_font_family(&style.family);
    result.set_font_size(style.size);
    if let Some(height) = style.height {
        result.set_height(height);
    }
    result
}

pub fn convert_font_weight(weight: drawing_api::FontWeight) -> impellers::FontWeight {
    match weight {
        drawing_api::FontWeight::Thin => impellers::FontWeight::Thin,
        drawing_api::FontWeight::ExtraLight => impellers::FontWeight::ExtraLight,
        drawing_api::FontWeight::Light => impellers::FontWeight::Light,
        drawing_api::FontWeight::Regular => impellers::FontWeight::Regular,
        drawing_api::FontWeight::Medium => impellers::FontWeight::Medium,
        drawing_api::FontWeight::SemiBold => impellers::FontWeight::SemiBold,
        drawing_api::FontWeight::Bold => impellers::FontWeight::Bold,
        drawing_api::FontWeight::ExtraBold => impellers::FontWeight::ExtraBold,
        drawing_api::FontWeight::Black => impellers::FontWeight::Black,
    }
}

pub fn convert_font_style(style: drawing_api::FontStyle) -> impellers::FontStyle {
    match style {
        drawing_api::FontStyle::Normal => impellers::FontStyle::Normal,
        drawing_api::FontStyle::Italic => impellers::FontStyle::Italic,
    }
}

pub fn convert_color_matrix(color_matrix: &drawing_api::ColorMatrix) -> impellers::ColorMatrix {
    impellers::ColorMatrix { m: color_matrix.m }
}

pub fn convert_matrix(matrix: &drawing_api::Matrix) -> impellers::Matrix {
    impellers::Matrix::from_array(matrix.to_array())
}
