use crate::{DipPoint, DipRect};

use super::FillType;

pub trait PathBuilder: Default {
    type Path;

    /// Sets the fill type.
    fn set_fill_type(&mut self, fill_type: FillType);

    /// Move the cursor to the specified location.
    fn move_to(&mut self, location: impl Into<DipPoint>);

    /// Add a line segment from the current cursor location to the given location.
    /// The cursor location is updated to be at the endpoint.
    fn line_to(&mut self, location: impl Into<DipPoint>);

    /// Add a quadratic bezier curve from whose start point is the cursor
    /// to the specified end point using the a single control point.
    /// The cursor location is updated to be at the endpoint.
    fn quadratic_curve_to(
        &mut self,
        control_point: impl Into<DipPoint>,
        end_point: impl Into<DipPoint>,
    ) {
        let control_point = control_point.into();
        self.bezier_curve_to(control_point.clone(), control_point, end_point);
    }

    /// Add a cubic bezier curve whose start point is current cursor location
    /// to the specified end point using the two specified control points.
    /// The cursor location is updated to be at the endpoint.
    fn bezier_curve_to(
        &mut self,
        control_point_1: impl Into<DipPoint>,
        control_point_2: impl Into<DipPoint>,
        end_point: impl Into<DipPoint>,
    );

    /// Adds a rectangle to the path.
    fn add_rect(&mut self, rect: impl Into<DipRect>) {
        let rect = rect.into();
        let tl = rect.origin.clone();
        let bl = DipPoint::new(rect.origin.x, rect.origin.y + rect.size.height);
        let br = DipPoint::new(
            rect.origin.x + rect.size.width,
            rect.origin.y + rect.size.height,
        );
        let tr = DipPoint::new(rect.origin.x + rect.size.width, rect.origin.y);
        self.move_to(tl);
        self.line_to(tr);
        self.line_to(br);
        self.line_to(bl);
        self.close();
    }

    /// Close the path.
    fn close(&mut self);

    /// Builds the path.
    fn build(self) -> Result<Self::Path, &'static str>;
}
