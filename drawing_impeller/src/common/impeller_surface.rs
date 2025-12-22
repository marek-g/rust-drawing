pub struct ImpellerSurface {
    pub(crate) surface: impellers::Surface,
}

impl drawing_api::Surface for ImpellerSurface {
    type DisplayList = crate::DisplayList;

    fn draw(&mut self, display_list: &Self::DisplayList) -> Result<(), &'static str> {
        self.surface.draw_display_list(&display_list.display_list)
    }

    fn present(self) -> Result<(), &'static str> {
        self.surface.present()
    }
}
