extern crate drawing;
extern crate gfx;

mod gfx_text;
mod font;

use self::drawing::font::*;
use self::drawing::color::*;
use self::drawing::units::*;

pub struct GfxTextFont<R: gfx::Resources, F: gfx::Factory<R>> {
    size: u8,
    renderer: gfx_text::Renderer<R, F>
}

impl<R: gfx::Resources, F: gfx::Factory<R>> GfxTextFont<R, F> {
    pub fn create(factory: F, bytes: &[u8], font_params: FontParams) -> Self {
        let size: u8 = font_params.size.get().trunc() as u8;
        let renderer = gfx_text::new(factory)
            .with_font_data(bytes)
            .with_size(size)
            .build()
            .unwrap(); 
        GfxTextFont {
            size: size,
            renderer: renderer
        }
    }

    pub fn draw<C: gfx::CommandBuffer<R>, T: gfx::format::RenderFormat>(&mut self,
        encoder: &mut gfx::Encoder<R, C>,
        target: &gfx::handle::RenderTargetView<R, T>) {
        self.renderer.draw(encoder, target);
    }
}

impl<R: gfx::Resources, F: gfx::Factory<R>> Font for GfxTextFont<R, F> {
    fn add_text(&mut self,
		color: &Color,
		text: &str,
		pos: Point,
		transform: UnknownToDeviceTransform) {
        self.renderer.add(text, [pos.x as i32, pos.y as i32], *color)
    }

    fn get_dimensions(&mut self,  text: &str) -> (u16, u16) {
        let dims = self.renderer.measure(text);
        (dims.0 as u16, dims.1 as u16)
    }
}
