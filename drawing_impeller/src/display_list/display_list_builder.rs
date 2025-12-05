use drawing_api::DipRect;

use super::{
    convert_clip_operation, convert_device_rect, convert_image_filter, convert_matrix,
    convert_point, convert_radii, convert_rect, convert_texture_sampling,
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

    fn scale(&mut self, x_scale: f32, y_scale: f32) {
        self.display_list_builder.scale(x_scale, y_scale);
    }

    fn rotate(&mut self, angle_degrees: f32) {
        self.display_list_builder.rotate(angle_degrees);
    }

    fn translate(&mut self, x_translation: f32, y_translation: f32) {
        self.display_list_builder
            .translate(x_translation, y_translation);
    }

    fn transform(&mut self, transform: &drawing_api::Matrix) {
        self.display_list_builder
            .transform(&convert_matrix(transform));
    }

    fn set_transform(&mut self, transform: &drawing_api::Matrix) {
        self.display_list_builder
            .set_transform(&convert_matrix(transform));
    }

    fn get_transform(&self) -> drawing_api::Matrix {
        drawing_api::Matrix::from_array(self.display_list_builder.get_transform().m)
    }

    fn reset_transform(&mut self) {
        self.display_list_builder.reset_transform();
    }

    fn clip_rect(&mut self, rect: impl Into<DipRect>, operation: drawing_api::ClipOperation) {
        let rect = convert_rect(&rect.into());
        let operation = convert_clip_operation(&operation);
        self.display_list_builder.clip_rect(&rect, operation);
    }

    fn clip_oval(
        &mut self,
        oval_bounds: impl Into<DipRect>,
        operation: drawing_api::ClipOperation,
    ) {
        let ovel_bounds = convert_rect(&oval_bounds.into());
        let operation = convert_clip_operation(&operation);
        self.display_list_builder.clip_oval(&ovel_bounds, operation);
    }

    fn clip_rounded_rect(
        &mut self,
        rect: impl Into<DipRect>,
        radii: &drawing_api::RoundingRadii,
        operation: drawing_api::ClipOperation,
    ) {
        let rect = convert_rect(&rect.into());
        let radii = convert_radii(&radii);
        let operation = convert_clip_operation(&operation);
        self.display_list_builder
            .clip_rounded_rect(&rect, &radii, operation);
    }

    fn clip_path(
        &mut self,
        path: &<Self::PathBuilder as drawing_api::PathBuilder>::Path,
        operation: drawing_api::ClipOperation,
    ) {
        let operation = convert_clip_operation(&operation);
        self.display_list_builder.clip_path(&path, operation);
    }

    fn save(&mut self) {
        self.display_list_builder.save();
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
            .draw_paragraph(&paragraph.paragraph, convert_point(&location.into()));
    }

    fn build(mut self) -> Result<Self::DisplayList, &'static str> {
        self.display_list_builder
            .build()
            .ok_or("Cannot build impeller display list")
    }
}
