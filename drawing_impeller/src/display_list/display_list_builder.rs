use drawing_api::DipRect;

use super::{
    convert_device_rect, convert_image_filter, convert_point, convert_rect,
    convert_texture_sampling,
};

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
        let bounds = bounds.into().map(|r| convert_rect(&r));
        Self {
            display_list_builder: impellers::DisplayListBuilder::new(bounds.as_ref()),
        }
    }

    fn save_layer(
        &mut self,
        bounds: impl Into<drawing_api::DipRect>,
        paint: Option<&Self::Paint>,
        filter: Option<drawing_api::ImageFilter>,
    ) {
        let bounds = convert_rect(&bounds.into());
        self.display_list_builder.save_layer(
            &bounds,
            paint.map(|p| &p.paint),
            filter.map(|f| convert_image_filter(f)).as_ref(),
        );
    }

    fn restore(&mut self) {
        self.display_list_builder.restore();
    }

    fn draw_paint(&mut self, paint: &Self::Paint) {
        self.display_list_builder.draw_paint(&paint.paint);
    }

    fn draw_line(
        &mut self,
        from: impl Into<drawing_api::DipPoint>,
        to: impl Into<drawing_api::DipPoint>,
        paint: &Self::Paint,
    ) {
        self.display_list_builder.draw_line(
            convert_point(&from.into()),
            convert_point(&to.into()),
            &paint.paint,
        );
    }

    fn draw_rect(&mut self, rect: impl Into<drawing_api::DipRect>, paint: &Self::Paint) {
        self.display_list_builder
            .draw_rect(&convert_rect(&rect.into()), &paint.paint);
    }

    fn draw_path(&mut self, path: &impellers::Path, paint: &Self::Paint) {
        self.display_list_builder.draw_path(path, &paint.paint);
    }

    fn draw_texture_rect(
        &mut self,
        texture: &Self::Texture,
        src_rect: impl Into<drawing_api::DeviceRect>,
        dst_rect: impl Into<drawing_api::DipRect>,
        sampling: drawing_api::TextureSampling,
        paint: Option<&Self::Paint>,
    ) {
        self.display_list_builder.draw_texture_rect(
            &texture.texture,
            &convert_device_rect(&src_rect.into()),
            &convert_rect(&dst_rect.into()),
            convert_texture_sampling(sampling),
            paint.map(|p| &p.paint),
        );

        /*let paint = Self::Paint::default();

        self.display_list_builder.draw_texture(
            &texture.texture,
            convert_point(&dst_rect.into().origin),
            convert_texture_sampling(sampling),
            &paint.paint,
        );*/
    }

    fn draw_paragraph(
        &mut self,
        location: impl Into<drawing_api::DipPoint>,
        paragraph: &<Self::ParagraphBuilder as drawing_api::ParagraphBuilder>::Paragraph,
    ) {
        self.display_list_builder
            .draw_paragraph(paragraph, convert_point(&location.into()));
    }

    fn build(mut self) -> Result<Self::DisplayList, &'static str> {
        self.display_list_builder
            .build()
            .ok_or("Cannot build impeller display list")
    }
}
