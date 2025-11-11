pub struct GlSurface;

impl drawing_api::Surface for GlSurface {
    type DisplayList = Vec<crate::generic::renderer::Primitive>;

    fn draw(&mut self, display_list: &Self::DisplayList) -> Result<(), &'static str> {
        todo!()
    }
}
