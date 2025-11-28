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
    type Path = (Vec<PathElement>, FillType);

    fn set_fill_type(&mut self, fill_type: FillType) {
        self.fill_type = fill_type;
    }

    fn move_to(&mut self, location: impl Into<drawing_api::DipPoint>) {
        let location = location.into();
        self.path
            .push(PathElement::MoveTo(PixelPoint::new(location.x, location.y)));
    }

    fn line_to(&mut self, location: impl Into<drawing_api::DipPoint>) {
        let location = location.into();
        self.path
            .push(PathElement::LineTo(PixelPoint::new(location.x, location.y)));
    }

    fn bezier_curve_to(
        &mut self,
        control_point_1: impl Into<drawing_api::DipPoint>,
        control_point_2: impl Into<drawing_api::DipPoint>,
        end_point: impl Into<drawing_api::DipPoint>,
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

    fn close(&mut self) {
        self.path.push(PathElement::ClosePath);
    }

    fn build(self) -> Result<Self::Path, &'static str> {
        Ok((self.path, self.fill_type))
    }
}
