pub struct PathBuilder;

impl Default for PathBuilder {
    fn default() -> Self {
        Self {}
    }
}

impl drawing_api::PathBuilder for PathBuilder {
    type Path = ();

    fn set_fill_type(&mut self, fill_type: drawing_api::FillType) {
        //todo!()
    }

    fn move_to(&mut self, location: impl Into<drawing_api::DipPoint>) {
        //todo!()
    }

    fn line_to(&mut self, location: impl Into<drawing_api::DipPoint>) {
        //todo!()
    }

    fn bezier_curve_to(
        &mut self,
        control_point_1: impl Into<drawing_api::DipPoint>,
        control_point_2: impl Into<drawing_api::DipPoint>,
        end_point: impl Into<drawing_api::DipPoint>,
    ) {
        //todo!()
    }

    fn close(&mut self) {
        //todo!()
    }

    fn build(self) -> Result<Self::Path, &'static str> {
        //todo!()
        Ok(())
    }
}
