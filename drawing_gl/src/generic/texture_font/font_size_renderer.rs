//! A library for drawing text for drawing graphics API.
//! Uses freetype-rs underneath to former the font bitmap texture and collect
//! information about face glyphs.
//!
//! # Examples
//!
//! Basic usage:
//!
//! ```ignore
//! // Initialize text renderer.
//! let mut text = FontSizeRenderer::new(&buffer, 25)?;
//!
//! // In render loop:
//!
//! // Add some text 10 pixels down and right from the top left screen corner.
//! text.add(
//!     "The quick brown fox jumps over the lazy dog",  // Text to add
//!     [10, 10],                                       // Position
//!     [0.65, 0.16, 0.16, 1.0],                        // Text color
//! );
//!
//! // Draw text.
//! text.draw_at(&mut device, &render_target, camera_projection).unwrap();
//! ```

#![deny(missing_docs)]

use crate::generic::clipping::clip_image;
use crate::generic::device::Device;
use crate::generic::device::TexturedY8Vertex;
use crate::generic::texture_font::bitmap_font::BitmapFont;
use drawing_api::ColorFormat;
use drawing_api::UnknownToDeviceTransform;

/// Text renderer.
///
/// # Examples
///
/// ```ignore
/// let mut file = File::open(font_path).unwrap();
/// let mut buffer = Vec::new();
/// file.read_to_end(&mut buffer);
///
/// let mut text = FontSizeRenderer::new(&buffer, 25);
/// ```
pub struct FontSizeRenderer<D: Device> {
    bitmap_font: BitmapFont,
    texture: Option<D::Texture>,
    vertex_data: Vec<TexturedY8Vertex>,
}

impl<D: Device> FontSizeRenderer<D> {
    pub fn new(font_data: &[u8], font_size: u8) -> Result<Self, &'static str> {
        let bitmap_font = BitmapFont::from_bytes(font_data, font_size, None)?;

        Ok(FontSizeRenderer {
            bitmap_font,
            texture: None,
            vertex_data: Vec::new(),
        })
    }

    /// Add some text to the current draw scene relative to the top left corner
    /// of the screen using pixel coordinates.
    pub fn add(&mut self, text: &str, pos: [i32; 2], clipping_rect: [f32; 4], color: [f32; 4]) {
        // `Result` is used here as an `Either` analogue.
        let (mut x, mut y) = (pos[0] as f32, pos[1] as f32);
        let line_height = self.bitmap_font.get_font_height() as f32;
        for ch in text.chars() {
            if ch == '\n' {
                x = pos[0] as f32;
                y = y + line_height;
            } else if ch == '\t' {
                if let Some(ch_info) = self.bitmap_font.find_char(' ') {
                    x += (ch_info.x_advance * 4) as f32;
                }
            } else {
                let ch_info = match self.bitmap_font.find_char(ch) {
                    Some(info) => info,
                    // Skip unknown chars from text string. Probably it would be
                    // better to place some "?" mark instead but it may not exist
                    // in the font too.
                    None => continue,
                };
                let x_offset = x + ch_info.x_offset as f32;
                let y_offset = y + ch_info.y_offset as f32;
                let tex = ch_info.tex;

                if let Some(clipped) = clip_image(
                    x_offset,
                    y_offset,
                    ch_info.width as f32,
                    ch_info.height as f32,
                    clipping_rect[0],
                    clipping_rect[1],
                    clipping_rect[2],
                    clipping_rect[3],
                    &[
                        tex[0],
                        tex[1],
                        tex[0] + ch_info.tex_width,
                        tex[1] + ch_info.tex_height,
                    ],
                ) {
                    Self::add_image(
                        &mut self.vertex_data,
                        clipped.0,
                        clipped.1,
                        clipped.2,
                        clipped.3,
                        clipped.4,
                        color,
                    );
                }

                x += ch_info.x_advance as f32;
            }
        }
    }

    /// Draw using provided projection matrix.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// text.add_at("Test1", [6.0, 0.0, 0.0], [1.0, 0.0, 0.0, 1.0]);
    /// text.add_at("Test2", [0.0, 5.0, 0.0], [0.0, 1.0, 0.0, 1.0]);
    /// text.draw_at(&mut device, &render_target, camera_projection).unwrap();
    /// ```
    pub fn draw_at(
        &mut self,
        device: &mut D,
        target: &D::RenderTarget,
        transform: UnknownToDeviceTransform,
    ) -> Result<(), &'static str> {
        if self.texture.is_none() {
            self.texture = Some(device.create_texture(
                self.bitmap_font.get_image(),
                self.bitmap_font.get_width(),
                self.bitmap_font.get_height(),
                ColorFormat::Y8,
            )?);
        }
        device.triangles_textured_y8(
            target,
            self.texture.as_ref().unwrap(),
            false,
            &self.vertex_data,
            transform,
        );
        self.vertex_data.clear();
        Ok(())
    }

    pub fn get_bitmap_font(&self) -> &BitmapFont {
        &self.bitmap_font
    }

    fn add_image(
        vertex_data: &mut Vec<TexturedY8Vertex>,
        x1: f32,
        y1: f32,
        width: f32,
        height: f32,
        uv: [f32; 4],
        color: [f32; 4],
    ) {
        // Top-left point, index + 0.
        let vert0 = TexturedY8Vertex::new([x1, y1], [uv[0], uv[1]], color);
        // Bottom-left point, index + 1.
        let vert1 = TexturedY8Vertex::new([x1, y1 + height], [uv[0], uv[3]], color);
        // Bottom-right point, index + 2.
        let vert2 = TexturedY8Vertex::new([x1 + width, y1 + height], [uv[2], uv[3]], color);
        // Top-right point, index + 3.
        let vert3 = TexturedY8Vertex::new([x1 + width, y1], [uv[2], uv[1]], color);

        // Top-left triangle.
        // 0--3
        // | /
        // |/
        // 1
        vertex_data.push(vert0);
        vertex_data.push(vert1);
        vertex_data.push(vert3);
        // Bottom-right triangle.
        //    3
        //   /|
        //  / |
        // 1--2
        vertex_data.push(vert3);
        vertex_data.push(vert1);
        vertex_data.push(vert2);
    }
}
