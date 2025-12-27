use std::any::Any;

use crate::{
    ClipOperation, Color, DisplayListBuilder, ImageFilter, Matrix, OptRef, PixelPoint, PixelRect,
    RoundingRadii, TextureSampling,
};

use super::{
    convert_image_filter, paragraph_object::ParagraphObject, path_object::PathObject,
    DisplayListObject, ImageFilterFragmentObject, PaintObject, TextureObject,
};

pub trait DisplayListBuilderObject {
    /// Apply a scale to the transformation matrix currently on top of the save stack.
    fn scale(&mut self, x_scale: f32, y_scale: f32);

    /// Apply a scale to the transformation matrix currently on top of the save stack.
    fn rotate(&mut self, angle_degrees: f32);

    /// Apply a translation to the transformation matrix currently on top of the save stack.
    fn translate(&mut self, x_translation: f32, y_translation: f32);

    /// Appends the the provided transformation to the transformation already on the save stack.
    fn transform(&mut self, transform: &Matrix);

    /// Clear the transformation on top of the save stack and replace it with a new value.
    fn set_transform(&mut self, transform: &Matrix);

    /// Get the transformation currently built up on the top of the transformation stack.
    fn get_transform(&self) -> Matrix;

    /// Reset the transformation on top of the transformation stack to identity.
    fn reset_transform(&mut self);

    /// Reduces the clip region to the intersection of the current clip and the given rectangle taking into account the clip operation.
    fn clip_rect(&mut self, rect: PixelRect, operation: ClipOperation);

    /// Reduces the clip region to the intersection of the current clip and the given oval taking into account the clip operation.
    fn clip_oval(&mut self, oval_bounds: PixelRect, operation: ClipOperation);

    /// Reduces the clip region to the intersection of the current clip and the given rounded rectangle taking into account the clip operation.
    fn clip_rounded_rect(
        &mut self,
        rect: PixelRect,
        radii: RoundingRadii,
        operation: ClipOperation,
    );

    /// Reduces the clip region to the intersection of the current clip and the given path taking into account the clip operation.
    fn clip_path(&mut self, path: &Box<dyn PathObject>, operation: ClipOperation);

    /// Stashes the current transformation and clip state onto a save stack.
    fn save(&mut self);

    /// Stashes the current transformation and clip state onto a save stack
    /// and creates and creates an offscreen layer
    /// onto which subsequent rendering intent will be directed to.
    fn save_layer(
        &mut self,
        bounds: PixelRect,
        paint: Option<&Box<dyn PaintObject>>,
        filter: Option<ImageFilter<Box<dyn ImageFilterFragmentObject>>>,
    );

    /// Gets the current size of the save stack.
    fn get_save_count(&mut self) -> usize;

    /// Pops the last entry pushed onto the save stack using a call to Self::save or Self::save_layer.
    fn restore(&mut self);

    /// Effectively calls restore till the size of the save stack becomes a specified count.
    fn restore_to_count(&mut self, count: usize) {
        while self.get_save_count() > count {
            self.restore();
        }
    }

    /// Fills the current clip with the specified paint.
    fn draw_paint(&mut self, paint: &Box<dyn PaintObject>);

    /// Draws a line segment.
    fn draw_line(&mut self, from: PixelPoint, to: PixelPoint, paint: &Box<dyn PaintObject>);

    /// Draws a dash line segment.
    fn draw_dashed_line(
        &mut self,
        from: PixelPoint,
        to: PixelPoint,
        on_length: f32,
        off_length: f32,
        paint: &Box<dyn PaintObject>,
    );

    /// Draws a rectangle.
    fn draw_rect(&mut self, rect: PixelRect, paint: &Box<dyn PaintObject>);

    /// Draws a rounded rectangle.
    fn draw_rounded_rect(
        &mut self,
        rect: PixelRect,
        radii: RoundingRadii,
        paint: &Box<dyn PaintObject>,
    );

    /// Draws a shape that is the different between the specified rectangles.
    fn draw_rounded_rect_difference(
        &mut self,
        outer_rect: PixelRect,
        outer_radii: RoundingRadii,
        inner_rect: PixelRect,
        inner_radii: RoundingRadii,
        paint: &Box<dyn PaintObject>,
    );

    /// Draws an oval.
    fn draw_oval(&mut self, oval_bounds: PixelRect, paint: &Box<dyn PaintObject>);

    /// Draws a path.
    fn draw_path(&mut self, path: &Box<dyn PathObject>, paint: &Box<dyn PaintObject>);

    /// Draws a shadow for a Path given a material elevation.
    fn draw_shadow(
        &mut self,
        path: &Box<dyn PathObject>,
        color: Color,
        elevation: f32,
        oocluder_is_transparent: bool,
        device_pixel_ratio: f32,
    );

    /// Draw a portion of texture at the specified location.
    fn draw_texture_rect(
        &mut self,
        texture: &Box<dyn TextureObject>,
        src_rect: PixelRect,
        dst_rect: PixelRect,
        sampling: TextureSampling,
        paint: Option<&Box<dyn PaintObject>>,
    );

    /// Draws a texture at the specified point.
    fn draw_texture(
        &mut self,
        texture: &Box<dyn TextureObject>,
        point: PixelPoint,
        sampling: TextureSampling,
        paint: Option<&Box<dyn PaintObject>>,
    );

    /// Draws a paragraph at the specified location.
    fn draw_paragraph(&mut self, location: PixelPoint, paragraph: &Box<dyn ParagraphObject>);

    /// Draws another display list.
    fn draw_display_list(&mut self, display_list: &Box<dyn DisplayListObject>, opacity: f32);

    /// Builds display list.
    fn build(self) -> Result<Box<dyn DisplayListObject>, &'static str>;
}

impl<B: DisplayListBuilder> DisplayListBuilderObject for B {
    fn scale(&mut self, x_scale: f32, y_scale: f32) {
        self.scale(x_scale, y_scale);
    }

    fn rotate(&mut self, angle_degrees: f32) {
        self.rotate(angle_degrees);
    }

    fn translate(&mut self, x_translation: f32, y_translation: f32) {
        self.translate(x_translation, y_translation);
    }

    fn transform(&mut self, transform: &Matrix) {
        self.transform(transform);
    }

    fn set_transform(&mut self, transform: &Matrix) {
        self.set_transform(transform);
    }

    fn get_transform(&self) -> Matrix {
        self.get_transform()
    }

    fn reset_transform(&mut self) {
        self.reset_transform();
    }

    fn clip_rect(&mut self, rect: PixelRect, operation: ClipOperation) {
        self.clip_rect(rect, operation);
    }

    fn clip_oval(&mut self, oval_bounds: PixelRect, operation: ClipOperation) {
        self.clip_oval(oval_bounds, operation);
    }

    fn clip_rounded_rect(
        &mut self,
        rect: PixelRect,
        radii: RoundingRadii,
        operation: ClipOperation,
    ) {
        self.clip_rounded_rect(rect, radii, operation);
    }

    fn clip_path(&mut self, path: &Box<dyn PathObject>, operation: ClipOperation) {
        let path = (path as &dyn Any)
            .downcast_ref::<<B::PathBuilder as crate::PathBuilder>::Path>()
            .unwrap();
        self.clip_path(path, operation);
    }

    fn save(&mut self) {
        self.save();
    }

    fn save_layer(
        &mut self,
        bounds: PixelRect,
        paint: Option<&Box<dyn PaintObject>>,
        filter: Option<ImageFilter<Box<dyn ImageFilterFragmentObject>>>,
    ) {
        let paint =
            paint.map(|p| OptRef::Borrowed((p as &dyn Any).downcast_ref::<B::Paint>().unwrap()));
        let filter = filter.map(|f| convert_image_filter::<B::ImageFilterFragment>(f));
        self.save_layer(bounds, paint, filter);
    }

    fn get_save_count(&mut self) -> usize {
        self.get_save_count()
    }

    fn restore(&mut self) {
        self.restore();
    }

    fn draw_paint(&mut self, paint: &Box<dyn PaintObject>) {
        let paint = (paint as &dyn Any).downcast_ref::<B::Paint>().unwrap();
        self.draw_paint(paint);
    }

    fn draw_line(&mut self, from: PixelPoint, to: PixelPoint, paint: &Box<dyn PaintObject>) {
        let paint = (paint as &dyn Any).downcast_ref::<B::Paint>().unwrap();
        self.draw_line(from, to, paint);
    }

    fn draw_dashed_line(
        &mut self,
        from: PixelPoint,
        to: PixelPoint,
        on_length: f32,
        off_length: f32,
        paint: &Box<dyn PaintObject>,
    ) {
        let paint = (paint as &dyn Any).downcast_ref::<B::Paint>().unwrap();
        self.draw_dashed_line(from, to, on_length, off_length, paint);
    }

    fn draw_rect(&mut self, rect: PixelRect, paint: &Box<dyn PaintObject>) {
        let paint = (paint as &dyn Any).downcast_ref::<B::Paint>().unwrap();
        self.draw_rect(rect, paint);
    }

    fn draw_rounded_rect(
        &mut self,
        rect: PixelRect,
        radii: RoundingRadii,
        paint: &Box<dyn PaintObject>,
    ) {
        let paint = (paint as &dyn Any).downcast_ref::<B::Paint>().unwrap();
        self.draw_rounded_rect(rect, radii, paint);
    }

    fn draw_rounded_rect_difference(
        &mut self,
        outer_rect: PixelRect,
        outer_radii: RoundingRadii,
        inner_rect: PixelRect,
        inner_radii: RoundingRadii,
        paint: &Box<dyn PaintObject>,
    ) {
        let paint = (paint as &dyn Any).downcast_ref::<B::Paint>().unwrap();
        self.draw_rounded_rect_difference(outer_rect, outer_radii, inner_rect, inner_radii, paint);
    }

    fn draw_oval(&mut self, oval_bounds: PixelRect, paint: &Box<dyn PaintObject>) {
        let paint = (paint as &dyn Any).downcast_ref::<B::Paint>().unwrap();
        self.draw_oval(oval_bounds, paint);
    }

    fn draw_path(&mut self, path: &Box<dyn PathObject>, paint: &Box<dyn PaintObject>) {
        let path = (path as &dyn Any)
            .downcast_ref::<<B::PathBuilder as crate::PathBuilder>::Path>()
            .unwrap();
        let paint = (paint as &dyn Any).downcast_ref::<B::Paint>().unwrap();
        self.draw_path(path, paint);
    }

    fn draw_shadow(
        &mut self,
        path: &Box<dyn PathObject>,
        color: Color,
        elevation: f32,
        oocluder_is_transparent: bool,
        device_pixel_ratio: f32,
    ) {
        let path = (path as &dyn Any)
            .downcast_ref::<<B::PathBuilder as crate::PathBuilder>::Path>()
            .unwrap();
        self.draw_shadow(
            path,
            color,
            elevation,
            oocluder_is_transparent,
            device_pixel_ratio,
        );
    }

    fn draw_texture_rect(
        &mut self,
        texture: &Box<dyn TextureObject>,
        src_rect: PixelRect,
        dst_rect: PixelRect,
        sampling: TextureSampling,
        paint: Option<&Box<dyn PaintObject>>,
    ) {
        let texture = (texture as &dyn Any).downcast_ref::<B::Texture>().unwrap();
        let paint =
            paint.map(|p| OptRef::Borrowed((p as &dyn Any).downcast_ref::<B::Paint>().unwrap()));
        self.draw_texture_rect(texture, src_rect, dst_rect, sampling, paint);
    }

    fn draw_texture(
        &mut self,
        texture: &Box<dyn TextureObject>,
        point: PixelPoint,
        sampling: TextureSampling,
        paint: Option<&Box<dyn PaintObject>>,
    ) {
        let texture = (texture as &dyn Any).downcast_ref::<B::Texture>().unwrap();
        let paint =
            paint.map(|p| OptRef::Borrowed((p as &dyn Any).downcast_ref::<B::Paint>().unwrap()));
        self.draw_texture(texture, point, sampling, paint);
    }

    fn draw_paragraph(&mut self, location: PixelPoint, paragraph: &Box<dyn ParagraphObject>) {
        let paragraph = (paragraph as &dyn Any)
            .downcast_ref::<<B::ParagraphBuilder as crate::ParagraphBuilder>::Paragraph>()
            .unwrap();
        self.draw_paragraph(location, paragraph);
    }

    fn draw_display_list(&mut self, display_list: &Box<dyn DisplayListObject>, opacity: f32) {
        let display_list = (display_list as &dyn Any)
            .downcast_ref::<B::DisplayList>()
            .unwrap();
        self.draw_display_list(display_list, opacity);
    }

    fn build(self) -> Result<Box<dyn DisplayListObject>, &'static str> {
        Ok(Box::new(self.build()?))
    }
}
