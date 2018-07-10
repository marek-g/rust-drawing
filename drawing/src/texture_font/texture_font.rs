use backend::*;
use font::*;
use color::*;
use units::*;
use texture_font::gfx_text::{ Renderer, RendererBuilder };

use std::collections::HashMap;

pub struct TextureFont<B: Backend> {
    bytes: Vec<u8>,
    font_renderers: HashMap<u8, Renderer<B>>
}

impl<B: Backend> TextureFont<B> {
    fn get_or_create_font_renderer(&mut self, backend: &mut B, size: u8) -> &mut Renderer<B> {
        if !self.font_renderers.contains_key(&size) {
            let renderer = self.create_font_renderer(backend, size);
            self.font_renderers.entry(size).or_insert(renderer)
        } else {
            self.font_renderers.get_mut(&size).unwrap()
        }
    }

    fn create_font_renderer(&self, backend: &mut B, size: u8) -> Renderer<B> {
        RendererBuilder::new()
            .with_font_data(&self.bytes)
            .with_size(size)
            .build::<B>(backend)
            .unwrap()
    }
}

impl<B: Backend> Font<B> for TextureFont<B> {
    fn create(_backend: &mut B, bytes: Vec<u8>) -> Self {
        TextureFont {
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
        let renderer = self.get_or_create_font_renderer(backend, font_params.size);
        renderer.add(text, [pos.x as i32, pos.y as i32], *color);
        renderer.draw_at(backend, target, transform);
    }

    fn get_dimensions(&mut self, backend: &mut B, params: FontParams, text: &str) -> (u16, u16) {
        let renderer = self.get_or_create_font_renderer(backend, params.size);
        let dims = renderer.measure(text);
        (dims.0 as u16, dims.1 as u16)
    }
}
