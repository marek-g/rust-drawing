use backend::*;
use font::*;
use color::*;
use units::*;
use texture_font::gfx_text::{ Renderer, RendererBuilder };

use std::collections::HashMap;

pub struct TextureFont<D: Device> {
    bytes: Vec<u8>,
    font_renderers: HashMap<u8, Renderer<D>>
}

impl<D: Device> TextureFont<D> {
    fn get_or_create_font_renderer(&mut self, device: &mut D, size: u8) -> &mut Renderer<D> {
        if !self.font_renderers.contains_key(&size) {
            let renderer = self.create_font_renderer(device, size);
            self.font_renderers.entry(size).or_insert(renderer)
        } else {
            self.font_renderers.get_mut(&size).unwrap()
        }
    }

    fn create_font_renderer(&self, device: &mut D, size: u8) -> Renderer<D> {
        RendererBuilder::new()
            .with_font_data(&self.bytes)
            .with_size(size)
            .build::<D>(device)
            .unwrap()
    }
}

impl<D: Device> Font<D> for TextureFont<D> {
    fn create(_device: &mut D, bytes: Vec<u8>) -> Self {
        TextureFont {
            bytes: bytes,
            font_renderers: HashMap::new()
        }
    }

    fn draw(&mut self, device: &mut D, target: &D::RenderTarget,
		color: &Color,
		text: &str,
		pos: Point,
		font_params: FontParams,
		transform: UnknownToDeviceTransform) {
        let renderer = self.get_or_create_font_renderer(device, font_params.size);
        renderer.add(text, [pos.x as i32, pos.y as i32], *color);
        renderer.draw_at(device, target, transform);
    }

    fn get_dimensions(&mut self, device: &mut D, params: FontParams, text: &str) -> (u16, u16) {
        let renderer = self.get_or_create_font_renderer(device, params.size);
        let dims = renderer.measure(text);
        (dims.0 as u16, dims.1 as u16)
    }
}
