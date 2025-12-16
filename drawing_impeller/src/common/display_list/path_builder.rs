use super::{convert_fill_type, convert_point, convert_radii, convert_rect};

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
    type Path = crate::Path;

    fn set_fill_type(&mut self, fill_type: drawing_api::FillType) {
        self.fill_type = convert_fill_type(fill_type)
    }

    fn move_to(&mut self, location: impl Into<drawing_api::PixelPoint>) {
        self.path_builder.move_to(convert_point(&location.into()));
    }

    fn line_to(&mut self, location: impl Into<drawing_api::PixelPoint>) {
        self.path_builder.line_to(convert_point(&location.into()));
    }

    fn cubic_curve_to(
        &mut self,
        control_point_1: impl Into<drawing_api::PixelPoint>,
        control_point_2: impl Into<drawing_api::PixelPoint>,
        end_point: impl Into<drawing_api::PixelPoint>,
    ) {
        self.path_builder.cubic_curve_to(
            convert_point(&control_point_1.into()),
            convert_point(&control_point_2.into()),
            convert_point(&end_point.into()),
        );
    }

    fn quadratic_curve_to(
        &mut self,
        control_point: impl Into<drawing_api::PixelPoint>,
        end_point: impl Into<drawing_api::PixelPoint>,
    ) {
        self.path_builder.quadratic_curve_to(
            convert_point(&control_point.into()),
            convert_point(&end_point.into()),
        );
    }

    fn add_rounded_rect(
        &mut self,
        rect: impl Into<drawing_api::PixelRect>,
        rounding_radii: &drawing_api::RoundingRadii,
    ) {
        let rect = convert_rect(&rect.into());
        let radii = convert_radii(&rounding_radii);
        self.path_builder.add_rounded_rect(&rect, &radii);
    }

    fn add_oval(&mut self, oval_bounds: impl Into<drawing_api::PixelRect>) {
        let oval_bounds = convert_rect(&oval_bounds.into());
        self.path_builder.add_oval(&oval_bounds);
    }

    fn add_arc(
        &mut self,
        oval_bounds: impl Into<drawing_api::PixelRect>,
        start_angle_degrees: f32,
        end_angle_degrees: f32,
    ) {
        let oval_bounds = convert_rect(&oval_bounds.into());
        self.path_builder
            .add_arc(&oval_bounds, start_angle_degrees, end_angle_degrees);
    }

    fn close(&mut self) {
        self.path_builder.close();
    }

    fn build(mut self) -> Result<Self::Path, &'static str> {
        Ok(crate::Path {
            path: self.path_builder.take_path_new(self.fill_type),
        })
    }

    fn build_copy(&mut self) -> Result<Self::Path, &'static str> {
        Ok(crate::Path {
            path: self.path_builder.copy_path_new(self.fill_type),
        })
    }
}
