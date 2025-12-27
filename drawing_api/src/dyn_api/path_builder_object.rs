use crate::{FillType, PathBuilder, PixelPoint, PixelRect, RoundingRadii};

use super::PathObject;

pub trait PathBuilderObject {
    /// Sets the fill type.
    fn set_fill_type(&mut self, fill_type: FillType);

    /// Move the cursor to the specified location.
    fn move_to(&mut self, location: PixelPoint);

    /// Add a line segment from the current cursor location to the given location.
    /// The cursor location is updated to be at the endpoint.
    fn line_to(&mut self, location: PixelPoint);

    /// Add a quadratic bezier curve from whose start point is the cursor
    /// to the specified end point using the a single control point.
    /// The cursor location is updated to be at the endpoint.
    fn quadratic_curve_to(&mut self, control_point: PixelPoint, end_point: PixelPoint);

    /// Add a cubic bezier curve whose start point is current cursor location
    /// to the specified end point using the two specified control points.
    /// The cursor location is updated to be at the endpoint.
    fn cubic_curve_to(
        &mut self,
        control_point_1: PixelPoint,
        control_point_2: PixelPoint,
        end_point: PixelPoint,
    );

    /// Adds a rectangle to the path.
    fn add_rect(&mut self, rect: PixelRect);

    /// Add a rounded rect with potentially non-uniform radii to the path.
    fn add_rounded_rect(&mut self, rect: PixelRect, rounding_radii: &RoundingRadii);

    /// Add an oval to the path.
    fn add_oval(&mut self, oval_bounds: PixelRect);

    /// Add an arc to the path.
    fn add_arc(&mut self, oval_bounds: PixelRect, start_angle_degrees: f32, end_angle_degrees: f32);

    /// Close the path.
    fn close(&mut self);

    /// Builds the path.
    fn build(self) -> Box<dyn PathObject>;

    /// Create a new path by copying the existing built-up path.
    /// The existing path can continue being added to.
    fn build_copy(&mut self) -> Box<dyn PathObject>;
}

impl<B: PathBuilder> PathBuilderObject for B {
    fn set_fill_type(&mut self, fill_type: FillType) {
        self.set_fill_type(fill_type);
    }

    fn move_to(&mut self, location: PixelPoint) {
        self.move_to(location);
    }

    fn line_to(&mut self, location: PixelPoint) {
        self.line_to(location);
    }

    fn quadratic_curve_to(&mut self, control_point: PixelPoint, end_point: PixelPoint) {
        self.quadratic_curve_to(control_point, end_point);
    }

    fn cubic_curve_to(
        &mut self,
        control_point_1: PixelPoint,
        control_point_2: PixelPoint,
        end_point: PixelPoint,
    ) {
        self.cubic_curve_to(control_point_1, control_point_2, end_point);
    }

    fn add_rect(&mut self, rect: PixelRect) {
        self.add_rect(rect);
    }

    fn add_rounded_rect(&mut self, rect: PixelRect, rounding_radii: &RoundingRadii) {
        self.add_rounded_rect(rect, rounding_radii);
    }

    fn add_oval(&mut self, oval_bounds: PixelRect) {
        self.add_oval(oval_bounds);
    }

    fn add_arc(
        &mut self,
        oval_bounds: PixelRect,
        start_angle_degrees: f32,
        end_angle_degrees: f32,
    ) {
        self.add_arc(oval_bounds, start_angle_degrees, end_angle_degrees);
    }

    fn close(&mut self) {
        self.close();
    }

    fn build(self) -> Box<dyn PathObject> {
        Box::new(self.build())
    }

    fn build_copy(&mut self) -> Box<dyn PathObject> {
        Box::new(self.build_copy())
    }
}
