pub struct ImpellerSurface {
    pub(crate) surface: impellers::Surface,
}

impl drawing_api::Surface for ImpellerSurface {
    type Context = crate::ImpellerContext;

    fn draw(
        &mut self,
        display_list: &<<Self::Context as drawing_api::Context>::DisplayListBuilder as drawing_api::DisplayListBuilder>::DisplayList,
    ) -> Result<(), &'static str> {
        self.surface.draw_display_list(display_list)
    }

    fn present(self) -> Result<(), &'static str> {
        self.surface.present()
    }
}
