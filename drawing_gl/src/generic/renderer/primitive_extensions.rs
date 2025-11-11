use crate::generic::renderer::*;
use drawing_api::*;

const KAPPA90: f32 = 0.5522847493;

pub fn rect_path<R: Into<PixelRect>>(rect: R) -> Vec<PathElement> {
    let mut res = Vec::with_capacity(5);
    let rect = rect.into();
    res.push(PathElement::MoveTo(PixelPoint::new(
        rect.origin.x,
        rect.origin.y,
    )));
    res.push(PathElement::LineTo(PixelPoint::new(
        rect.origin.x,
        rect.origin.y + rect.size.height,
    )));
    res.push(PathElement::LineTo(PixelPoint::new(
        rect.origin.x + rect.size.width,
        rect.origin.y + rect.size.height,
    )));
    res.push(PathElement::LineTo(PixelPoint::new(
        rect.origin.x + rect.size.width,
        rect.origin.y,
    )));
    res.push(PathElement::ClosePath);
    res
}

pub fn rect_path_rounded<R: Into<PixelRect>>(rect: R, radius: f32) -> Vec<PathElement> {
    rect_rounded_varying_path(rect, radius, radius, radius, radius)
}

pub fn rect_path_rounded_half<R: Into<PixelRect>>(
    rect: R,
    radius: f32,
    is_upper_left_half: bool,
) -> Vec<PathElement> {
    rect_rounded_varying_path_half(rect, radius, radius, radius, radius, is_upper_left_half)
}

pub fn rect_rounded_varying_path<R: Into<PixelRect>>(
    rect: R,
    lt: f32,
    rt: f32,
    rb: f32,
    lb: f32,
) -> Vec<PathElement> {
    let rect = rect.into();
    if lt < 0.1 && rt < 0.1 && lb < 0.1 && rb < 0.1 {
        rect_path(rect)
    } else {
        let halfw = rect.size.width.abs() * 0.5;
        let halfh = rect.size.height.abs() * 0.5;
        let rxlb = lb.min(halfw) * rect.size.width.signum();
        let rylb = lb.min(halfh) * rect.size.height.signum();
        let rxrb = rb.min(halfw) * rect.size.width.signum();
        let ryrb = rb.min(halfh) * rect.size.height.signum();
        let rxrt = rt.min(halfw) * rect.size.width.signum();
        let ryrt = rt.min(halfh) * rect.size.height.signum();
        let rxlt = lt.min(halfw) * rect.size.width.signum();
        let rylt = lt.min(halfh) * rect.size.height.signum();

        vec![
            PathElement::MoveTo(PixelPoint::new(rect.origin.x, rect.origin.y + rylt)),
            PathElement::LineTo(PixelPoint::new(
                rect.origin.x,
                rect.origin.y + rect.size.height - rylb,
            )),
            PathElement::BezierTo(
                PixelPoint::new(
                    rect.origin.x,
                    rect.origin.y + rect.size.height - rylb * (1.0 - KAPPA90),
                ),
                PixelPoint::new(
                    rect.origin.x + rxlb * (1.0 - KAPPA90),
                    rect.origin.y + rect.size.height,
                ),
                PixelPoint::new(rect.origin.x + rxlb, rect.origin.y + rect.size.height),
            ),
            PathElement::LineTo(PixelPoint::new(
                rect.origin.x + rect.size.width - rxrb,
                rect.origin.y + rect.size.height,
            )),
            PathElement::BezierTo(
                PixelPoint::new(
                    rect.origin.x + rect.size.width - rxrb * (1.0 - KAPPA90),
                    rect.origin.y + rect.size.height,
                ),
                PixelPoint::new(
                    rect.origin.x + rect.size.width,
                    rect.origin.y + rect.size.height - ryrb * (1.0 - KAPPA90),
                ),
                PixelPoint::new(
                    rect.origin.x + rect.size.width,
                    rect.origin.y + rect.size.height - ryrb,
                ),
            ),
            PathElement::LineTo(PixelPoint::new(
                rect.origin.x + rect.size.width,
                rect.origin.y + ryrt,
            )),
            PathElement::BezierTo(
                PixelPoint::new(
                    rect.origin.x + rect.size.width,
                    rect.origin.y + ryrt * (1.0 - KAPPA90),
                ),
                PixelPoint::new(
                    rect.origin.x + rect.size.width - rxrt * (1.0 - KAPPA90),
                    rect.origin.y,
                ),
                PixelPoint::new(rect.origin.x + rect.size.width - rxrt, rect.origin.y),
            ),
            PathElement::LineTo(PixelPoint::new(rect.origin.x + rxlt, rect.origin.y)),
            PathElement::BezierTo(
                PixelPoint::new(rect.origin.x + rxlt * (1.0 - KAPPA90), rect.origin.y),
                PixelPoint::new(rect.origin.x, rect.origin.y + rylt * (1.0 - KAPPA90)),
                PixelPoint::new(rect.origin.x, rect.origin.y + rylt),
            ),
            PathElement::ClosePath,
        ]
    }
}

pub fn rect_rounded_varying_path_half<R: Into<PixelRect>>(
    rect: R,
    lt: f32,
    rt: f32,
    rb: f32,
    lb: f32,
    is_upper_left_half: bool,
) -> Vec<PathElement> {
    let rect = rect.into();
    if lt < 0.1 && rt < 0.1 && lb < 0.1 && rb < 0.1 {
        rect_path(rect)
    } else {
        let halfw = rect.size.width.abs() * 0.5;
        let halfh = rect.size.height.abs() * 0.5;
        let rxlb = lb.min(halfw) * rect.size.width.signum();
        let rylb = lb.min(halfh) * rect.size.height.signum();
        let rxrb = rb.min(halfw) * rect.size.width.signum();
        let ryrb = rb.min(halfh) * rect.size.height.signum();
        let rxrt = rt.min(halfw) * rect.size.width.signum();
        let ryrt = rt.min(halfh) * rect.size.height.signum();
        let rxlt = lt.min(halfw) * rect.size.width.signum();
        let rylt = lt.min(halfh) * rect.size.height.signum();

        if is_upper_left_half {
            vec![
                PathElement::MoveTo(PixelPoint::new(
                    rect.origin.x + rect.size.width,
                    rect.origin.y + ryrt,
                )),
                PathElement::BezierTo(
                    PixelPoint::new(
                        rect.origin.x + rect.size.width,
                        rect.origin.y + ryrt * (1.0 - KAPPA90),
                    ),
                    PixelPoint::new(
                        rect.origin.x + rect.size.width - rxrt * (1.0 - KAPPA90),
                        rect.origin.y,
                    ),
                    PixelPoint::new(rect.origin.x + rect.size.width - rxrt, rect.origin.y),
                ),
                PathElement::LineTo(PixelPoint::new(rect.origin.x + rxlt, rect.origin.y)),
                PathElement::BezierTo(
                    PixelPoint::new(rect.origin.x + rxlt * (1.0 - KAPPA90), rect.origin.y),
                    PixelPoint::new(rect.origin.x, rect.origin.y + rylt * (1.0 - KAPPA90)),
                    PixelPoint::new(rect.origin.x, rect.origin.y + rylt),
                ),
                PathElement::LineTo(PixelPoint::new(
                    rect.origin.x,
                    rect.origin.y + rect.size.height - rylb,
                )),
                PathElement::BezierTo(
                    PixelPoint::new(
                        rect.origin.x,
                        rect.origin.y + rect.size.height - rylb * (1.0 - KAPPA90),
                    ),
                    PixelPoint::new(
                        rect.origin.x + rxlb * (1.0 - KAPPA90),
                        rect.origin.y + rect.size.height,
                    ),
                    PixelPoint::new(rect.origin.x + rxlb, rect.origin.y + rect.size.height),
                ),
            ]
        } else {
            vec![
                PathElement::MoveTo(PixelPoint::new(
                    rect.origin.x + rxlb,
                    rect.origin.y + rect.size.height,
                )),
                PathElement::LineTo(PixelPoint::new(
                    rect.origin.x + rect.size.width - rxrb,
                    rect.origin.y + rect.size.height,
                )),
                PathElement::BezierTo(
                    PixelPoint::new(
                        rect.origin.x + rect.size.width - rxrb * (1.0 - KAPPA90),
                        rect.origin.y + rect.size.height,
                    ),
                    PixelPoint::new(
                        rect.origin.x + rect.size.width,
                        rect.origin.y + rect.size.height - ryrb * (1.0 - KAPPA90),
                    ),
                    PixelPoint::new(
                        rect.origin.x + rect.size.width,
                        rect.origin.y + rect.size.height - ryrb,
                    ),
                ),
                PathElement::LineTo(PixelPoint::new(
                    rect.origin.x + rect.size.width,
                    rect.origin.y + ryrt,
                )),
            ]
        }
    }
}

pub fn circle_path<P: Into<PixelPoint>>(center: P, radius: f32) -> Vec<PathElement> {
    ellipse_path(center, radius, radius)
}

pub fn ellipse_path<P: Into<PixelPoint>>(
    center: P,
    radius_x: f32,
    radius_y: f32,
) -> Vec<PathElement> {
    let center = center.into();
    vec![
        PathElement::MoveTo(PixelPoint::new(center.x - radius_x, center.y)),
        PathElement::BezierTo(
            PixelPoint::new(center.x - radius_x, center.y + radius_y * KAPPA90),
            PixelPoint::new(center.x - radius_x * KAPPA90, center.y + radius_y),
            PixelPoint::new(center.x, center.y + radius_y),
        ),
        PathElement::BezierTo(
            PixelPoint::new(center.x + radius_x * KAPPA90, center.y + radius_y),
            PixelPoint::new(center.x + radius_x, center.y + radius_y * KAPPA90),
            PixelPoint::new(center.x + radius_x, center.y),
        ),
        PathElement::BezierTo(
            PixelPoint::new(center.x + radius_x, center.y - radius_y * KAPPA90),
            PixelPoint::new(center.x + radius_x * KAPPA90, center.y - radius_y),
            PixelPoint::new(center.x, center.y - radius_y),
        ),
        PathElement::BezierTo(
            PixelPoint::new(center.x - radius_x * KAPPA90, center.y - radius_y),
            PixelPoint::new(center.x - radius_x, center.y - radius_y * KAPPA90),
            PixelPoint::new(center.x - radius_x, center.y),
        ),
        PathElement::ClosePath,
    ]
}

/// Creates a path that represents pixel aligned rectangle path to be used with stroke.
///
/// # Arguments
///
/// * `rect` - marks the boundaries
/// * `thickness` - thickness of the outline, the outline grows inside the rect
pub fn pixel_rect_path<R: Into<PixelRect>, T: Into<PixelThickness>>(
    rect: R,
    thickness: T,
) -> Vec<PathElement> {
    let rect = rect.into();
    let thickness = thickness.into().get();

    rect_path(PixelRect::new(
        PixelPoint::new(
            rect.origin.x + thickness * 0.5f32,
            rect.origin.y + thickness * 0.5f32,
        ),
        PixelSize::new(rect.size.width - thickness, rect.size.height - thickness),
    ))
}

pub fn pixel_rect_path_rounded<R: Into<PixelRect>, T: Into<PixelThickness>>(
    rect: R,
    thickness: T,
    radius: f32,
) -> Vec<PathElement> {
    let rect = rect.into();
    let thickness = thickness.into().get();

    rect_path_rounded(
        PixelRect::new(
            PixelPoint::new(
                rect.origin.x + thickness * 0.5f32,
                rect.origin.y + thickness * 0.5f32,
            ),
            PixelSize::new(rect.size.width - thickness, rect.size.height - thickness),
        ),
        radius,
    )
}

pub fn pixel_rect_path_rounded_half<R: Into<PixelRect>, T: Into<PixelThickness>>(
    rect: R,
    thickness: T,
    radius: f32,
    is_upper_left_half: bool,
) -> Vec<PathElement> {
    let rect = rect.into();
    let thickness = thickness.into().get();

    rect_path_rounded_half(
        PixelRect::new(
            PixelPoint::new(
                rect.origin.x + thickness * 0.5f32,
                rect.origin.y + thickness * 0.5f32,
            ),
            PixelSize::new(rect.size.width - thickness, rect.size.height - thickness),
        ),
        radius,
        is_upper_left_half,
    )
}

/// Creates a path that represents pixel aligned horizontal line path to be used with stroke.
///
/// # Arguments
///
/// * `start` - starting point (left top pixel coordinates)
/// * `length` - length of the line in pixels
/// * `thickness` - thickness of the line, the outline grows to the bottom
pub fn pixel_horizontal_line_path<P: Into<PixelPoint>, T: Into<PixelThickness>>(
    start: P,
    length: T,
    thickness: T,
) -> Vec<PathElement> {
    let start = start.into();
    let length = length.into().get();
    let thickness = thickness.into().get();

    vec![
        PathElement::MoveTo(PixelPoint::new(start.x, start.y + thickness * 0.5f32)),
        PathElement::LineTo(PixelPoint::new(
            start.x + length,
            start.y + thickness * 0.5f32,
        )),
    ]
}

/// Creates a path that represents pixel aligned vertical line path to be used with stroke.
///
/// # Arguments
///
/// * `start` - starting point (left top pixel coordinates)
/// * `length` - length of the line in pixels
/// * `thickness` - thickness of the line, the outline grows to the right
pub fn pixel_vertical_line_path<P: Into<PixelPoint>, T: Into<PixelThickness>>(
    start: P,
    length: T,
    thickness: T,
) -> Vec<PathElement> {
    let start = start.into();
    let length = length.into().get();
    let thickness = thickness.into().get();

    vec![
        PathElement::MoveTo(PixelPoint::new(start.x + thickness * 0.5f32, start.y)),
        PathElement::LineTo(PixelPoint::new(
            start.x + thickness * 0.5f32,
            start.y + length,
        )),
    ]
}
