use anyhow::Result;

use crate::backend::*;
use crate::color::*;
use crate::font::*;
use crate::texture_font::gfx_text::{Renderer, RendererBuilder};
use crate::units::*;

use std::collections::HashMap;

pub struct TextureFont<D: Device> {
    bytes: Vec<u8>,
    font_renderers: HashMap<u8, Renderer<D>>,
}

impl<D: Device> TextureFont<D> {
    fn get_or_create_font_renderer(
        &mut self,
        device: &mut D,
        size: u8,
    ) -> Result<&mut Renderer<D>> {
        if !self.font_renderers.contains_key(&size) {
            let renderer = self.create_font_renderer(device, size)?;
            Ok(self.font_renderers.entry(size).or_insert(renderer))
        } else {
            Ok(self.font_renderers.get_mut(&size).unwrap())
        }
    }

    fn create_font_renderer(&self, device: &mut D, size: u8) -> Result<Renderer<D>> {
        Ok(RendererBuilder::new()
            .with_font_data(&self.bytes)
            .with_size(size)
            .build::<D>(device)?)
    }
}

impl<D: Device> Font<D> for TextureFont<D> {
    fn create(_device: &mut D, bytes: Vec<u8>) -> Result<Self> {
        Ok(TextureFont {
            bytes: bytes,
            font_renderers: HashMap::new(),
        })
    }

    fn draw(
        &mut self,
        device: &mut D,
        target: &D::RenderTarget,
        color: &Color,
        text: &str,
        pos: Point,
        clipping_rect: Rect,
        font_params: FontParams,
        transform: UnknownToDeviceTransform,
    ) -> Result<()> {
        let renderer = self.get_or_create_font_renderer(device, font_params.size)?;
        renderer.add(
            text,
            [pos.x as i32, pos.y as i32],
            [
                clipping_rect.origin.x,
                clipping_rect.origin.y,
                clipping_rect.size.width,
                clipping_rect.size.height,
            ],
            *color,
        );
        renderer.draw_at(device, target, transform)?;
        Ok(())
    }

    fn get_dimensions(
        &mut self,
        device: &mut D,
        params: FontParams,
        text: &str,
    ) -> Result<(u16, u16)> {
        let renderer = self.get_or_create_font_renderer(device, params.size)?;
        let dims = renderer.measure(text);
        Ok((dims.0 as u16, dims.1 as u16))
    }

    fn get_dimensions_each_char(
        &mut self,
        device: &mut D,
        params: FontParams,
        text: &str,
    ) -> Result<(Vec<i16>, u16)> {
        let renderer = self.get_or_create_font_renderer(device, params.size)?;
        let dims = renderer.measure_each_char(text);
        Ok((dims.0, dims.1 as u16))
    }
}
