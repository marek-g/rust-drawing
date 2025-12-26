use drawing_api::{FillType, PixelPoint};

use super::PathElement;

pub struct PathBuilder {
    path: Vec<PathElement>,
    fill_type: FillType,
}

impl Default for PathBuilder {
    fn default() -> Self {
        Self {
            path: Vec::new(),
            fill_type: FillType::NonZero,
        }
    }
}

impl drawing_api::PathBuilder for PathBuilder {
    type Path = crate::display_list::Path;

    fn set_fill_type(&mut self, fill_type: FillType) {
        self.fill_type = fill_type;
    }

    fn move_to(&mut self, location: impl Into<drawing_api::PixelPoint>) {
        let location = location.into();
        self.path
            .push(PathElement::MoveTo(PixelPoint::new(location.x, location.y)));
    }

    fn line_to(&mut self, location: impl Into<drawing_api::PixelPoint>) {
        let location = location.into();
        self.path
            .push(PathElement::LineTo(PixelPoint::new(location.x, location.y)));
    }

    fn cubic_curve_to(
        &mut self,
        control_point_1: impl Into<drawing_api::PixelPoint>,
        control_point_2: impl Into<drawing_api::PixelPoint>,
        end_point: impl Into<drawing_api::PixelPoint>,
    ) {
        let control_point_1 = control_point_1.into();
        let control_point_2 = control_point_2.into();
        let end_point = end_point.into();
        self.path.push(PathElement::BezierTo(
            PixelPoint::new(control_point_1.x, control_point_1.y),
            PixelPoint::new(control_point_2.x, control_point_2.y),
            PixelPoint::new(end_point.x, end_point.y),
        ));
    }

    fn add_rounded_rect(
        &mut self,
        rect: impl Into<drawing_api::PixelRect>,
        rounding_radii: &drawing_api::RoundingRadii,
    ) {
        todo!()
    }

    fn add_oval(&mut self, oval_bounds: impl Into<drawing_api::PixelRect>) {
        todo!()
    }

    fn add_arc(
        &mut self,
        oval_bounds: impl Into<drawing_api::PixelRect>,
        start_angle_degrees: f32,
        end_angle_degrees: f32,
    ) {
        todo!()
    }

    fn close(&mut self) {
        self.path.push(PathElement::ClosePath);
    }

    fn build(self) -> Self::Path {
        super::Path {
            path: self.path,
            fill_type: self.fill_type,
        }
    }

    fn build_copy(&mut self) -> Self::Path {
        super::Path {
            path: self.path.clone(),
            fill_type: self.fill_type,
        }
    }
}
