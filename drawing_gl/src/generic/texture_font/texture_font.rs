use font_size_renderer::FontSizeRenderer;

use crate::generic::device::*;
use crate::generic::texture_font::*;
use crate::units::PixelToDeviceTransform;
use drawing_api::*;

use std::collections::HashMap;

pub struct TextureFont<D: Device> {
    bytes: Vec<u8>,
    font_renderers: HashMap<u8, FontSizeRenderer<D>>,
}

impl<D: Device> TextureFont<D> {
    fn get_or_create_font_renderer(
        &mut self,
        size: u8,
    ) -> Result<&mut FontSizeRenderer<D>, &'static str> {
        if !self.font_renderers.contains_key(&size) {
            let renderer = self.create_font_renderer(size)?;
            Ok(self.font_renderers.entry(size).or_insert(renderer))
        } else {
            Ok(self.font_renderers.get_mut(&size).unwrap())
        }
    }

    fn create_font_renderer(&self, size: u8) -> Result<FontSizeRenderer<D>, &'static str> {
        Ok(FontSizeRenderer::new(&self.bytes, size)?)
    }
}

impl<D: Device> Font<D> for TextureFont<D> {
    fn create(bytes: Vec<u8>) -> Result<Self, &'static str> {
        Ok(TextureFont {
            bytes,
            font_renderers: HashMap::new(),
        })
    }

    fn draw(
        &mut self,
        device: &mut D,
        target: &D::RenderTarget,
        color: &crate::generic::device::Color,
        text: &str,
        pos: PixelPoint,
        clipping_rect: Option<PixelRect>,
        font_params: FontParams,
        transform: PixelToDeviceTransform,
    ) -> Result<(), &'static str> {
        let renderer = self.get_or_create_font_renderer(font_params.size)?;
        renderer.add(
            text,
            [pos.x as i32, pos.y as i32],
            clipping_rect.map(|r| [r.origin.x, r.origin.y, r.size.width, r.size.height]),
            *color,
        );
        renderer.draw_at(device, target, transform)?;
        Ok(())
    }

    fn get_dimensions(
        &mut self,
        params: FontParams,
        text: &str,
    ) -> Result<(u16, u16), &'static str> {
        let renderer = self.get_or_create_font_renderer(params.size)?;
        let dims = renderer.get_bitmap_font().measure(text);
        Ok((dims.0 as u16, dims.1 as u16))
    }

    fn get_dimensions_each_char(
        &mut self,
        params: FontParams,
        text: &str,
    ) -> Result<(Vec<i16>, u16), &'static str> {
        let renderer = self.get_or_create_font_renderer(params.size)?;
        let dims = renderer.get_bitmap_font().measure_each_char(text);
        Ok((dims.0, dims.1 as u16))
    }
}
