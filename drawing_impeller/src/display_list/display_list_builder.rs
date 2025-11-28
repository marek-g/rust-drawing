use drawing_api::DipRect;

pub struct DisplayListBuilder {
    pub(crate) display_list_builder: impellers::DisplayListBuilder,
}

impl drawing_api::DisplayListBuilder for DisplayListBuilder {
    type DisplayList = impellers::DisplayList;

    type Paint = crate::Paint;

    type ParagraphBuilder = crate::ParagraphBuilder;

    type PathBuilder = crate::PathBuilder;

    type Texture = crate::ImpellerTexture;

    fn new(bounds: impl Into<Option<DipRect>>) -> Self {
        // TODO:
        //let bounds = bounds.into().map(|b| {});
        Self {
            display_list_builder: impellers::DisplayListBuilder::new(None),
        }
    }

    fn save_layer(
        &mut self,
        bounds: impl Into<drawing_api::DipRect>,
        paint: Option<&Self::Paint>,
        filter: Option<drawing_api::ImageFilter>,
    ) {
        //todo!()
    }

    fn restore(&mut self) {
        //todo!()
    }

    fn draw_paint(&mut self, paint: &Self::Paint) {
        //todo!()
    }

    fn draw_line(
        &mut self,
        from: impl Into<drawing_api::DipPoint>,
        to: impl Into<drawing_api::DipPoint>,
        paint: &Self::Paint,
    ) {
        //todo!()
    }

    fn draw_rect(&mut self, rect1: impl Into<drawing_api::DipRect>, paint: &Self::Paint) {
        let rect1 = rect1.into();

        self.display_list_builder.draw_rect(
            &impellers::Rect::new(
                impellers::Point::new(rect1.origin.x, rect1.origin.y),
                impellers::Size::new(rect1.size.width, rect1.size.height),
            ),
            &paint.paint,
        );
    }

    fn draw_path(&mut self, path: &(), paint: &Self::Paint) {
        //todo!()
    }

    fn draw_texture_rect(
        &mut self,
        texture: &Self::Texture,
        src_rect: impl Into<drawing_api::DeviceRect>,
        dst_rect: impl Into<drawing_api::DipRect>,
        sampling: drawing_api::TextureSampling,
        paint: Option<&Self::Paint>,
    ) {
        //todo!()
    }

    fn draw_paragraph(
        &mut self,
        location: impl Into<drawing_api::DipPoint>,
        paragraph: &<Self::ParagraphBuilder as drawing_api::ParagraphBuilder>::Paragraph,
    ) {
        //todo!()
    }

    fn build(self) -> Result<Self::DisplayList, &'static str> {
        self.display_list_builder
            .build()
            .ok_or("Cannot build impeller display list")
    }
}
