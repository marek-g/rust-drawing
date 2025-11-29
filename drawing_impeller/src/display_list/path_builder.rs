use super::{convert_fill_type, convert_point};

pub struct PathBuilder {
    pub(crate) path_builder: impellers::PathBuilder,
    pub(crate) fill_type: impellers::FillType,
}

impl Default for PathBuilder {
    fn default() -> Self {
        Self {
            path_builder: impellers::PathBuilder::default(),
            fill_type: impellers::FillType::NonZero,
        }
    }
}

impl drawing_api::PathBuilder for PathBuilder {
    type Path = impellers::Path;

    fn set_fill_type(&mut self, fill_type: drawing_api::FillType) {
        self.fill_type = convert_fill_type(fill_type)
    }

    fn move_to(&mut self, location: impl Into<drawing_api::DipPoint>) {
        self.path_builder.move_to(convert_point(&location.into()));
    }

    fn line_to(&mut self, location: impl Into<drawing_api::DipPoint>) {
        self.path_builder.line_to(convert_point(&location.into()));
    }

    fn bezier_curve_to(
        &mut self,
        control_point_1: impl Into<drawing_api::DipPoint>,
        control_point_2: impl Into<drawing_api::DipPoint>,
        end_point: impl Into<drawing_api::DipPoint>,
    ) {
        self.path_builder.cubic_curve_to(
            convert_point(&control_point_1.into()),
            convert_point(&control_point_2.into()),
            convert_point(&end_point.into()),
        );
    }

    fn quadratic_curve_to(
        &mut self,
        control_point: impl Into<drawing_api::DipPoint>,
        end_point: impl Into<drawing_api::DipPoint>,
    ) {
        self.path_builder.quadratic_curve_to(
            convert_point(&control_point.into()),
            convert_point(&end_point.into()),
        );
    }

    fn close(&mut self) {
        self.path_builder.close();
    }

    fn build(mut self) -> Result<Self::Path, &'static str> {
        Ok(self.path_builder.take_path_new(self.fill_type))
    }
}
