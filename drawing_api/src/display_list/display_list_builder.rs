use crate::{Matrix, PixelPoint, PixelRect};

use super::{ClipOperation, Color, ImageFilter, RoundingRadii, TextureSampling};

pub trait DisplayListBuilder {
    type DisplayList;
    type FragmentShader: crate::FragmentShader;
    type Paint: crate::Paint;
    type ParagraphBuilder: crate::ParagraphBuilder;
    type PathBuilder: crate::PathBuilder;
    type Texture: crate::Texture;

    /// Create a new display list builder.
    /// An optional cull rectangle may be specified.
    fn new(bounds: impl Into<Option<PixelRect>>) -> Self;

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
    fn clip_rect(&mut self, rect: impl Into<PixelRect>, operation: ClipOperation);

    /// Reduces the clip region to the intersection of the current clip and the given oval taking into account the clip operation.
    fn clip_oval(&mut self, oval_bounds: impl Into<PixelRect>, operation: ClipOperation);

    /// Reduces the clip region to the intersection of the current clip and the given rounded rectangle taking into account the clip operation.
    fn clip_rounded_rect(
        &mut self,
        rect: impl Into<PixelRect>,
        radii: &RoundingRadii,
        operation: ClipOperation,
    );

    /// Reduces the clip region to the intersection of the current clip and the given path taking into account the clip operation.
    fn clip_path(
        &mut self,
        path: &<Self::PathBuilder as crate::PathBuilder>::Path,
        operation: ClipOperation,
    );

    /// Stashes the current transformation and clip state onto a save stack.
    fn save(&mut self);

    /// Stashes the current transformation and clip state onto a save stack
    /// and creates and creates an offscreen layer
    /// onto which subsequent rendering intent will be directed to.
    fn save_layer(
        &mut self,
        bounds: impl Into<PixelRect>,
        paint: Option<&Self::Paint>,
        filter: Option<ImageFilter<Self::Texture, Self::FragmentShader>>,
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
    fn draw_paint(&mut self, paint: &Self::Paint);

    /// Draws a line segment.
    fn draw_line(
        &mut self,
        from: impl Into<PixelPoint>,
        to: impl Into<PixelPoint>,
        paint: &Self::Paint,
    );

    /// Draws a dash line segment.
    fn draw_dashed_line(
        &mut self,
        from: impl Into<PixelPoint>,
        to: impl Into<PixelPoint>,
        on_length: f32,
        off_length: f32,
        paint: &Self::Paint,
    );

    /// Draws a rectangle.
    fn draw_rect(&mut self, rect: impl Into<PixelRect>, paint: &Self::Paint);

    /// Draws a rounded rectangle.
    fn draw_rounded_rect(
        &mut self,
        rect: impl Into<PixelRect>,
        radii: &RoundingRadii,
        paint: &Self::Paint,
    );

    /// Draws a shape that is the different between the specified rectangles.
    fn draw_rounded_rect_difference(
        &mut self,
        outer_rect: impl Into<PixelRect>,
        outer_radii: &RoundingRadii,
        inner_rect: impl Into<PixelRect>,
        inner_radii: &RoundingRadii,
        paint: &Self::Paint,
    );

    /// Draws an oval.
    fn draw_oval(&mut self, oval_bounds: impl Into<PixelRect>, paint: &Self::Paint);

    /// Draws a path.
    fn draw_path(
        &mut self,
        path: &<Self::PathBuilder as crate::PathBuilder>::Path,
        paint: &Self::Paint,
    );

    /// Draws a shadow for a Path given a material elevation.
    fn draw_shadow(
        &mut self,
        path: &<Self::PathBuilder as crate::PathBuilder>::Path,
        color: &Color,
        elevation: f32,
        oocluder_is_transparent: bool,
        device_pixel_ratio: f32,
    );

    /// Draw a portion of texture at the specified location.
    fn draw_texture_rect(
        &mut self,
        texture: &Self::Texture,
        src_rect: impl Into<PixelRect>,
        dst_rect: impl Into<PixelRect>,
        sampling: TextureSampling,
        paint: Option<&Self::Paint>,
    );

    /// Draws a texture at the specified point.
    fn draw_texture(
        &mut self,
        texture: &Self::Texture,
        point: impl Into<PixelPoint>,
        sampling: TextureSampling,
        paint: Option<&Self::Paint>,
    );

    /// Draws a paragraph at the specified location.
    fn draw_paragraph(
        &mut self,
        location: impl Into<PixelPoint>,
        paragraph: &<Self::ParagraphBuilder as crate::ParagraphBuilder>::Paragraph,
    );

    /// Draws another display list.
    fn draw_display_list(&mut self, display_list: &Self::DisplayList, opacity: f32);

    /// Builds display list.
    fn build(self) -> Result<Self::DisplayList, &'static str>;
}
