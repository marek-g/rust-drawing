extern crate drawing;
extern crate gfx;

mod gfx_text;
mod font;

use self::drawing::backend::*;
use self::drawing::font::*;
use self::drawing::color::*;
use self::drawing::units::*;
use ::backend::GfxBackendExt;

use std::collections::HashMap;

pub struct GfxTextFont<R: gfx::Resources, F: gfx::Factory<R>> {
    bytes: Vec<u8>,
    font_renderers: HashMap<u8, gfx_text::Renderer<R, F>>
}

impl<R: gfx::Resources, F: gfx::Factory<R>> GfxTextFont<R, F> {
    fn get_or_create_font_renderer(&mut self, factory: F, size: u8) -> &mut gfx_text::Renderer<R, F> {
        if !self.font_renderers.contains_key(&size) {
            let renderer = self.create_font_renderer(factory, size);
            self.font_renderers.entry(size).or_insert(renderer)
        } else {
            self.font_renderers.get_mut(&size).unwrap()
        }
    }

    fn create_font_renderer(&self, factory: F, size: u8) -> gfx_text::Renderer<R, F> {
        gfx_text::new(factory)
            .with_font_data(&self.bytes)
            .with_size(size)
            .build()
            .unwrap()
    }
}

impl<R, F, B, ColorFormat> Font<B> for GfxTextFont<R, F> where
    R: gfx::Resources,
    F: gfx::Factory<R>,
    B: Backend<Factory = F, RenderTarget = gfx::handle::RenderTargetView<R, ColorFormat>> + GfxBackendExt<R>,
    ColorFormat: gfx::format::Formatted,
    <ColorFormat as gfx::format::Formatted>::Surface: gfx::format::RenderSurface,
    <ColorFormat as gfx::format::Formatted>::Channel: gfx::format::RenderChannel {
    fn create(_backend: &mut B, bytes: Vec<u8>) -> Self {
        GfxTextFont {
            bytes: bytes,
            font_renderers: HashMap::new()
        }
    }

    fn draw(&mut self, backend: &mut B, target: &B::RenderTarget,
		color: &Color,
		text: &str,
		pos: Point,
		font_params: FontParams,
		transform: UnknownToDeviceTransform) {
        let renderer = self.get_or_create_font_renderer(backend.get_factory(), font_params.size);
        renderer.add(text, [pos.x as i32, pos.y as i32], *color);
        renderer.draw(backend.get_encoder(), target);
    }

    fn get_dimensions(&mut self, backend: &mut B, params: FontParams, text: &str) -> (u16, u16) {
        let renderer = self.get_or_create_font_renderer(backend.get_factory(), params.size);
        let dims = renderer.measure(text);
        (dims.0 as u16, dims.1 as u16)
    }
}
