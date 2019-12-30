use crate::primitive::*;
use crate::units::*;

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

pub fn rounded_rect_path<R: Into<PixelRect>>(rect: R, radius: f32) -> Vec<PathElement> {
    rounded_rect_varying_path(rect, radius, radius, radius, radius)
}

pub fn rounded_rect_varying_path<R: Into<PixelRect>>(
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

        let mut res = Vec::with_capacity(10);
        res.push(PathElement::MoveTo(PixelPoint::new(
            rect.origin.x,
            rect.origin.y + rylt,
        )));
        res.push(PathElement::LineTo(PixelPoint::new(
            rect.origin.x,
            rect.origin.y + rect.size.height - rylb,
        )));
        res.push(PathElement::BezierTo(
            PixelPoint::new(
                rect.origin.x,
                rect.origin.y + rect.size.height - rylb * (1.0 - KAPPA90),
            ),
            PixelPoint::new(
                rect.origin.x + rxlb * (1.0 - KAPPA90),
                rect.origin.y + rect.size.height,
            ),
            PixelPoint::new(rect.origin.x + rxlb, rect.origin.y + rect.size.height),
        ));
        res.push(PathElement::LineTo(PixelPoint::new(
            rect.origin.x + rect.size.width - rxrb,
            rect.origin.y + rect.size.height,
        )));
        res.push(PathElement::BezierTo(
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
        ));
        res.push(PathElement::LineTo(PixelPoint::new(
            rect.origin.x + rect.size.width,
            rect.origin.y + ryrt,
        )));
        res.push(PathElement::BezierTo(
            PixelPoint::new(
                rect.origin.x + rect.size.width,
                rect.origin.y + ryrt * (1.0 - KAPPA90),
            ),
            PixelPoint::new(
                rect.origin.x + rect.size.width - rxrt * (1.0 - KAPPA90),
                rect.origin.y,
            ),
            PixelPoint::new(rect.origin.x + rect.size.width - rxrt, rect.origin.y),
        ));
        res.push(PathElement::LineTo(PixelPoint::new(
            rect.origin.x + rxlt,
            rect.origin.y,
        )));
        res.push(PathElement::BezierTo(
            PixelPoint::new(rect.origin.x + rxlt * (1.0 - KAPPA90), rect.origin.y),
            PixelPoint::new(rect.origin.x, rect.origin.y + rylt * (1.0 - KAPPA90)),
            PixelPoint::new(rect.origin.x, rect.origin.y + rylt),
        ));
        res.push(PathElement::ClosePath);
        res
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
    let mut res = Vec::with_capacity(6);
    let center = center.into();
    res.push(PathElement::MoveTo(PixelPoint::new(
        center.x - radius_x,
        center.y,
    )));
    res.push(PathElement::BezierTo(
        PixelPoint::new(center.x - radius_x, center.y + radius_y * KAPPA90),
        PixelPoint::new(center.x - radius_x * KAPPA90, center.y + radius_y),
        PixelPoint::new(center.x, center.y + radius_y),
    ));
    res.push(PathElement::BezierTo(
        PixelPoint::new(center.x + radius_x * KAPPA90, center.y + radius_y),
        PixelPoint::new(center.x + radius_x, center.y + radius_y * KAPPA90),
        PixelPoint::new(center.x + radius_x, center.y),
    ));
    res.push(PathElement::BezierTo(
        PixelPoint::new(center.x + radius_x, center.y - radius_y * KAPPA90),
        PixelPoint::new(center.x + radius_x * KAPPA90, center.y - radius_y),
        PixelPoint::new(center.x, center.y - radius_y),
    ));
    res.push(PathElement::BezierTo(
        PixelPoint::new(center.x - radius_x * KAPPA90, center.y - radius_y),
        PixelPoint::new(center.x - radius_x, center.y - radius_y * KAPPA90),
        PixelPoint::new(center.x - radius_x, center.y),
    ));
    res.push(PathElement::ClosePath);
    res
}
