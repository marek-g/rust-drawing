/// A surface represents a render target.
/// That can be usually a window or a texture.
pub trait Surface {
    type Context: crate::Context;

    /// Draws a display list on the surface.
    fn draw(
        &mut self,
        display_list: &<<Self::Context as crate::Context>::DisplayListBuilder as crate::DisplayListBuilder>::DisplayList,
    ) -> Result<(), &'static str>;

    // Present the surface to the underlying window system (for Vulkan).
    fn present(self) -> Result<(), &'static str>;
}
