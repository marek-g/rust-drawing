use crate::DipPoint;

pub trait PathBuilder {
    /// Move the cursor to the specified location.
    fn move_to(&mut self, location: impl Into<DipPoint>);

    /// Add a line segment from the current cursor location to the given location.
    /// The cursor location is updated to be at the endpoint.
    fn line_to(&mut self, location: impl Into<DipPoint>);

    /// Add a quadratic curve from whose start point is the cursor
    /// to the specified end point using the a single control point.
    /// The cursor location is updated to be at the endpoint.
    fn quadratic_curve_to(
        &mut self,
        control_point: impl Into<DipPoint>,
        end_point: impl Into<DipPoint>,
    );

    /// Add a cubic bezier curve whose start point is current cursor location
    /// to the specified end point using the two specified control points.
    /// The cursor location is updated to be at the endpoint.
    fn bezier_curve_to(
        &mut self,
        control_point_1: impl Into<DipPoint>,
        control_point_2: impl Into<DipPoint>,
        end_point: impl Into<DipPoint>,
    );

    /// Close the path.
    fn close(&mut self);
}
