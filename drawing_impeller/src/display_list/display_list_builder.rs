use drawing_api::PixelRect;

use crate::{ImpellerFragmentShader, ImpellerTexture};

use super::{
    convert_clip_operation, convert_color, convert_image_filter, convert_matrix, convert_point,
    convert_radii, convert_rect, convert_texture_sampling,
};

pub struct DisplayListBuilder {
    pub(crate) display_list_builder: impellers::DisplayListBuilder,
}

impl drawing_api::DisplayListBuilder for DisplayListBuilder {
    type DisplayList = impellers::DisplayList;

    type FragmentShader = crate::ImpellerFragmentShader;

    type Paint = crate::Paint;

    type ParagraphBuilder = crate::ParagraphBuilder;

    type PathBuilder = crate::PathBuilder;

    type Texture = crate::ImpellerTexture;

    fn new(bounds: impl Into<Option<PixelRect>>) -> Self {
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

    fn clip_rect(&mut self, rect: impl Into<PixelRect>, operation: drawing_api::ClipOperation) {
        let rect = convert_rect(&rect.into());
        let operation = convert_clip_operation(&operation);
        self.display_list_builder.clip_rect(&rect, operation);
    }

    fn clip_oval(
        &mut self,
        oval_bounds: impl Into<PixelRect>,
        operation: drawing_api::ClipOperation,
    ) {
        let ovel_bounds = convert_rect(&oval_bounds.into());
        let operation = convert_clip_operation(&operation);
        self.display_list_builder.clip_oval(&ovel_bounds, operation);
    }

    fn clip_rounded_rect(
        &mut self,
        rect: impl Into<PixelRect>,
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
        self.display_list_builder.clip_path(&path.path, operation);
    }

    fn save(&mut self) {
        self.display_list_builder.save();
    }

    fn save_layer(
        &mut self,
        bounds: impl Into<drawing_api::PixelRect>,
        paint: Option<&Self::Paint>,
        filter: Option<drawing_api::ImageFilter<ImpellerTexture, ImpellerFragmentShader>>,
    ) {
        let bounds = convert_rect(&bounds.into());
        self.display_list_builder.save_layer(
            &bounds,
            paint.map(|p| &p.paint),
            filter.map(|f| convert_image_filter(f)).as_ref(),
        );
    }

    fn get_save_count(&mut self) -> usize {
        self.display_list_builder.get_save_count() as usize
    }

    fn restore(&mut self) {
        self.display_list_builder.restore();
    }

    fn restore_to_count(&mut self, count: usize) {
        self.display_list_builder.restore_to_count(count as u32);
    }

    fn draw_paint(&mut self, paint: &Self::Paint) {
        self.display_list_builder.draw_paint(&paint.paint);
    }

    fn draw_line(
        &mut self,
        from: impl Into<drawing_api::PixelPoint>,
        to: impl Into<drawing_api::PixelPoint>,
        paint: &Self::Paint,
    ) {
        self.display_list_builder.draw_line(
            convert_point(&from.into()),
            convert_point(&to.into()),
            &paint.paint,
        );
    }

    fn draw_dashed_line(
        &mut self,
        from: impl Into<drawing_api::PixelPoint>,
        to: impl Into<drawing_api::PixelPoint>,
        on_length: f32,
        off_length: f32,
        paint: &Self::Paint,
    ) {
        self.display_list_builder.draw_dashed_line(
            convert_point(&from.into()),
            convert_point(&to.into()),
            on_length,
            off_length,
            &paint.paint,
        );
    }

    fn draw_rect(&mut self, rect: impl Into<drawing_api::PixelRect>, paint: &Self::Paint) {
        self.display_list_builder
            .draw_rect(&convert_rect(&rect.into()), &paint.paint);
    }

    fn draw_rounded_rect(
        &mut self,
        rect: impl Into<PixelRect>,
        radii: &drawing_api::RoundingRadii,
        paint: &Self::Paint,
    ) {
        self.display_list_builder.draw_rounded_rect(
            &convert_rect(&rect.into()),
            &convert_radii(&radii),
            &paint.paint,
        );
    }

    fn draw_rounded_rect_difference(
        &mut self,
        outer_rect: impl Into<PixelRect>,
        outer_radii: &drawing_api::RoundingRadii,
        inner_rect: impl Into<PixelRect>,
        inner_radii: &drawing_api::RoundingRadii,
        paint: &Self::Paint,
    ) {
        self.display_list_builder.draw_rounded_rect_difference(
            &convert_rect(&outer_rect.into()),
            &convert_radii(&outer_radii),
            &convert_rect(&inner_rect.into()),
            &convert_radii(&inner_radii),
            &paint.paint,
        );
    }

    fn draw_oval(&mut self, oval_bounds: impl Into<PixelRect>, paint: &Self::Paint) {
        self.display_list_builder
            .draw_rect(&convert_rect(&oval_bounds.into()), &paint.paint);
    }

    fn draw_path(
        &mut self,
        path: &<Self::PathBuilder as drawing_api::PathBuilder>::Path,
        paint: &Self::Paint,
    ) {
        self.display_list_builder
            .draw_path(&path.path, &paint.paint);
    }

    fn draw_shadow(
        &mut self,
        path: &<Self::PathBuilder as drawing_api::PathBuilder>::Path,
        color: &drawing_api::Color,
        elevation: f32,
        oocluder_is_transparent: bool,
        device_pixel_ratio: f32,
    ) {
        self.display_list_builder.draw_shadow(
            &path.path,
            &convert_color(color),
            elevation,
            oocluder_is_transparent,
            device_pixel_ratio,
        );
    }

    fn draw_texture(
        &mut self,
        texture: &Self::Texture,
        point: impl Into<drawing_api::PixelPoint>,
        sampling: drawing_api::TextureSampling,
        paint: Option<&Self::Paint>,
    ) {
        let paint = match paint {
            Some(paint) => &paint.paint,
            None => &impellers::Paint::default(),
        };

        self.display_list_builder.draw_texture(
            &texture.texture,
            convert_point(&point.into()),
            convert_texture_sampling(sampling),
            &paint,
        );
    }

    fn draw_texture_rect(
        &mut self,
        texture: &Self::Texture,
        src_rect: impl Into<drawing_api::PixelRect>,
        dst_rect: impl Into<drawing_api::PixelRect>,
        sampling: drawing_api::TextureSampling,
        paint: Option<&Self::Paint>,
    ) {
        self.display_list_builder.draw_texture_rect(
            &texture.texture,
            &convert_rect(&src_rect.into()),
            &convert_rect(&dst_rect.into()),
            convert_texture_sampling(sampling),
            paint.map(|p| &p.paint),
        );
    }

    fn draw_paragraph(
        &mut self,
        location: impl Into<drawing_api::PixelPoint>,
        paragraph: &<Self::ParagraphBuilder as drawing_api::ParagraphBuilder>::Paragraph,
    ) {
        self.display_list_builder
            .draw_paragraph(&paragraph.paragraph, convert_point(&location.into()));
    }

    fn draw_display_list(&mut self, display_list: &Self::DisplayList, opacity: f32) {
        self.display_list_builder
            .draw_display_list(display_list, opacity);
    }

    fn build(mut self) -> Result<Self::DisplayList, &'static str> {
        self.display_list_builder
            .build()
            .ok_or("Cannot build impeller display list")
    }
}
