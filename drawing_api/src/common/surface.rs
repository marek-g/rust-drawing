/// A surface represents a render target.
/// That can be usually a window or a texture.
pub trait Surface {
    type DisplayList;

    /// Draws a display list on the surface.
    fn draw(&mut self, display_list: &Self::DisplayList) -> Result<(), &'static str>;

    // Present the surface to the underlying window system (for Vulkan).
    fn present(self) -> Result<(), &'static str>;
}
