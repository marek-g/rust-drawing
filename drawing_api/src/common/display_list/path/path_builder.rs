use crate::{PixelPoint, PixelRect, RoundingRadii};

use super::FillType;

pub trait PathBuilder: Default + 'static {
    type Path: crate::Path;

    /// Sets the fill type.
    fn set_fill_type(&mut self, fill_type: FillType);

    /// Move the cursor to the specified location.
    fn move_to(&mut self, location: impl Into<PixelPoint>);

    /// Add a line segment from the current cursor location to the given location.
    /// The cursor location is updated to be at the endpoint.
    fn line_to(&mut self, location: impl Into<PixelPoint>);

    /// Add a quadratic bezier curve from whose start point is the cursor
    /// to the specified end point using the a single control point.
    /// The cursor location is updated to be at the endpoint.
    fn quadratic_curve_to(
        &mut self,
        control_point: impl Into<PixelPoint>,
        end_point: impl Into<PixelPoint>,
    ) {
        let control_point = control_point.into();
        self.cubic_curve_to(control_point.clone(), control_point, end_point);
    }

    /// Add a cubic bezier curve whose start point is current cursor location
    /// to the specified end point using the two specified control points.
    /// The cursor location is updated to be at the endpoint.
    fn cubic_curve_to(
        &mut self,
        control_point_1: impl Into<PixelPoint>,
        control_point_2: impl Into<PixelPoint>,
        end_point: impl Into<PixelPoint>,
    );

    /// Adds a rectangle to the path.
    fn add_rect(&mut self, rect: impl Into<PixelRect>) {
        let rect = rect.into();
        let tl = rect.origin.clone();
        let bl = PixelPoint::new(rect.origin.x, rect.origin.y + rect.size.height);
        let br = PixelPoint::new(
            rect.origin.x + rect.size.width,
            rect.origin.y + rect.size.height,
        );
        let tr = PixelPoint::new(rect.origin.x + rect.size.width, rect.origin.y);
        self.move_to(tl);
        self.line_to(tr);
        self.line_to(br);
        self.line_to(bl);
        self.close();
    }

    /// Add a rounded rect with potentially non-uniform radii to the path.
    fn add_rounded_rect(&mut self, rect: impl Into<PixelRect>, rounding_radii: &RoundingRadii);

    /// Add an oval to the path.
    fn add_oval(&mut self, oval_bounds: impl Into<PixelRect>);

    /// Add an arc to the path.
    fn add_arc(
        &mut self,
        oval_bounds: impl Into<PixelRect>,
        start_angle_degrees: f32,
        end_angle_degrees: f32,
    );

    /// Close the path.
    fn close(&mut self);

    /// Builds the path.
    fn build(self) -> Self::Path;

    /// Create a new path by copying the existing built-up path.
    /// The existing path can continue being added to.
    fn build_copy(&mut self) -> Self::Path;
}
