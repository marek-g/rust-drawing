use drawing_api::{OptRef, PixelRect, RoundingRadii};

use super::{
    convert_clip_operation, convert_color, convert_image_filter, convert_matrix, convert_point,
    convert_radii, convert_rect, convert_texture_sampling, ImageFilterFragment,
};

pub struct DisplayListBuilder {
    pub(crate) display_list_builder: impellers::DisplayListBuilder,
}

impl drawing_api::DisplayListBuilder for DisplayListBuilder {
    type DisplayList = impellers::DisplayList;

    type ImageFilterFragment = crate::ImageFilterFragment;

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
        convert_matrix(&self.display_list_builder.get_transform())
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
        let oval_bounds = convert_rect(&oval_bounds.into());
        let operation = convert_clip_operation(&operation);
        self.display_list_builder.clip_oval(&oval_bounds, operation);
    }

    fn clip_rounded_rect<'a>(
        &mut self,
        rect: impl Into<PixelRect>,
        radii: impl Into<OptRef<'a, RoundingRadii>>,
        operation: drawing_api::ClipOperation,
    ) {
        let rect = convert_rect(&rect.into());
        let radii = convert_radii(&radii.into());
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

    fn save_layer<'a>(
        &mut self,
        bounds: impl Into<drawing_api::PixelRect>,
        paint: impl Into<Option<OptRef<'a, Self::Paint>>>,
        filter: Option<drawing_api::ImageFilter<ImageFilterFragment>>,
    ) {
        let bounds = convert_rect(&bounds.into());
        let paint = paint.into();
        self.display_list_builder.save_layer(
            &bounds,
            paint.as_ref().map(|p| &p.paint),
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

    fn draw_paint<'a>(&mut self, paint: impl Into<OptRef<'a, Self::Paint>>) {
        let paint = paint.into();
        self.display_list_builder.draw_paint(&paint.paint);
    }

    fn draw_line<'a>(
        &mut self,
        from: impl Into<drawing_api::PixelPoint>,
        to: impl Into<drawing_api::PixelPoint>,
        paint: impl Into<OptRef<'a, Self::Paint>>,
    ) {
        let paint = paint.into();
        self.display_list_builder.draw_line(
            convert_point(&from.into()),
            convert_point(&to.into()),
            &paint.paint,
        );
    }

    fn draw_dashed_line<'a>(
        &mut self,
        from: impl Into<drawing_api::PixelPoint>,
        to: impl Into<drawing_api::PixelPoint>,
        on_length: f32,
        off_length: f32,
        paint: impl Into<OptRef<'a, Self::Paint>>,
    ) {
        let paint = paint.into();
        self.display_list_builder.draw_dashed_line(
            convert_point(&from.into()),
            convert_point(&to.into()),
            on_length,
            off_length,
            &paint.paint,
        );
    }

    fn draw_rect<'a>(
        &mut self,
        rect: impl Into<drawing_api::PixelRect>,
        paint: impl Into<OptRef<'a, Self::Paint>>,
    ) {
        let paint = paint.into();
        self.display_list_builder
            .draw_rect(&convert_rect(&rect.into()), &paint.paint);
    }

    fn draw_rounded_rect<'a>(
        &mut self,
        rect: impl Into<PixelRect>,
        radii: impl Into<OptRef<'a, RoundingRadii>>,
        paint: impl Into<OptRef<'a, Self::Paint>>,
    ) {
        let paint = paint.into();
        self.display_list_builder.draw_rounded_rect(
            &convert_rect(&rect.into()),
            &convert_radii(&radii.into()),
            &paint.paint,
        );
    }

    fn draw_rounded_rect_difference<'a>(
        &mut self,
        outer_rect: impl Into<PixelRect>,
        outer_radii: impl Into<OptRef<'a, RoundingRadii>>,
        inner_rect: impl Into<PixelRect>,
        inner_radii: impl Into<OptRef<'a, RoundingRadii>>,
        paint: impl Into<OptRef<'a, Self::Paint>>,
    ) {
        let paint = paint.into();
        self.display_list_builder.draw_rounded_rect_difference(
            &convert_rect(&outer_rect.into()),
            &convert_radii(&outer_radii.into()),
            &convert_rect(&inner_rect.into()),
            &convert_radii(&inner_radii.into()),
            &paint.paint,
        );
    }

    fn draw_oval<'a>(
        &mut self,
        oval_bounds: impl Into<PixelRect>,
        paint: impl Into<OptRef<'a, Self::Paint>>,
    ) {
        let paint = paint.into();
        self.display_list_builder
            .draw_rect(&convert_rect(&oval_bounds.into()), &paint.paint);
    }

    fn draw_path<'a>(
        &mut self,
        path: &<Self::PathBuilder as drawing_api::PathBuilder>::Path,
        paint: impl Into<OptRef<'a, Self::Paint>>,
    ) {
        let paint = paint.into();
        self.display_list_builder
            .draw_path(&path.path, &paint.paint);
    }

    fn draw_shadow(
        &mut self,
        path: &<Self::PathBuilder as drawing_api::PathBuilder>::Path,
        color: impl Into<drawing_api::Color>,
        elevation: f32,
        oocluder_is_transparent: bool,
        device_pixel_ratio: f32,
    ) {
        self.display_list_builder.draw_shadow(
            &path.path,
            &convert_color(&color.into()),
            elevation,
            oocluder_is_transparent,
            device_pixel_ratio,
        );
    }

    fn draw_texture<'a>(
        &mut self,
        texture: &Self::Texture,
        point: impl Into<drawing_api::PixelPoint>,
        sampling: drawing_api::TextureSampling,
        paint: impl Into<Option<OptRef<'a, Self::Paint>>>,
    ) {
        let paint = paint.into();
        let paint = paint.unwrap_or_else(|| {
            OptRef::Owned(Self::Paint {
                paint: impellers::Paint::default(),
            })
        });
        self.display_list_builder.draw_texture(
            &texture.texture,
            convert_point(&point.into()),
            convert_texture_sampling(sampling),
            &paint.paint,
        );
    }

    fn draw_texture_rect<'a>(
        &mut self,
        texture: &Self::Texture,
        src_rect: impl Into<drawing_api::PixelRect>,
        dst_rect: impl Into<drawing_api::PixelRect>,
        sampling: drawing_api::TextureSampling,
        paint: impl Into<Option<OptRef<'a, Self::Paint>>>,
    ) {
        let paint = paint.into();
        self.display_list_builder.draw_texture_rect(
            &texture.texture,
            &convert_rect(&src_rect.into()),
            &convert_rect(&dst_rect.into()),
            convert_texture_sampling(sampling),
            paint.as_ref().map(|p| &p.paint),
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
