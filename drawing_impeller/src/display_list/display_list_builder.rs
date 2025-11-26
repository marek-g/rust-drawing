pub struct DisplayListBuilder;

impl drawing_api::DisplayListBuilder for DisplayListBuilder {
    type DisplayList = ();

    type Paint = crate::Paint;

    type Paragraph = ();

    type Path = ();

    type Texture = crate::ImpellerTexture;

    fn save_layer(
        &mut self,
        bounds: impl Into<drawing_api::DipRect>,
        paint: Option<&Self::Paint>,
        filter: Option<drawing_api::ImageFilter>,
    ) {
        todo!()
    }

    fn restore(&mut self) {
        todo!()
    }

    fn draw_paint(&mut self, paint: &Self::Paint) {
        todo!()
    }

    fn draw_line(
        &mut self,
        from: impl Into<drawing_api::DipPoint>,
        to: impl Into<drawing_api::DipPoint>,
        paint: &Self::Paint,
    ) {
        todo!()
    }

    fn draw_rect(&mut self, rect: impl Into<drawing_api::DipRect>, paint: &Self::Paint) {
        todo!()
    }

    fn draw_path(&mut self, path: &Self::Path, paint: &Self::Paint) {
        todo!()
    }

    fn draw_texture_rect(
        &mut self,
        texture: &Self::Texture,
        src_rect: impl Into<drawing_api::DeviceRect>,
        dst_rect: impl Into<drawing_api::DipRect>,
        sampling: drawing_api::TextureSampling,
        paint: Option<&Self::Paint>,
    ) {
        todo!()
    }

    fn draw_paragraph(
        &mut self,
        location: impl Into<drawing_api::DipPoint>,
        paragraph: &Self::Paragraph,
    ) {
        todo!()
    }

    fn build(self) -> Result<Self::DisplayList, &'static str> {
        todo!()
    }
}
