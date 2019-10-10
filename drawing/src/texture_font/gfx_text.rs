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
//! let mut text = RendererBuilder::new()
//!                                .with_font_data(&self.bytes)
//!                                .with_size(size)
//!                                .build::<D>(device)
//!                                .unwrap()
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

extern crate freetype;
extern crate failure;

use crate::units::UnknownToDeviceTransform;
use crate::backend::TexturedY8Vertex;
use crate::color::ColorFormat;
use crate::backend::Device;
use crate::texture_font::font::BitmapFont;
pub use crate::texture_font::font::FontError;

const DEFAULT_FONT_SIZE: u8 = 16;

#[cfg(feature = "include-font")]
const DEFAULT_FONT_DATA: Option<&'static [u8]> =
    Some(include_bytes!("../assets/NotoSans-Regular.ttf"));
#[cfg(not(feature = "include-font"))]
const DEFAULT_FONT_DATA: Option<&'static [u8]> =
    None;

/// General error type returned by the library. Wraps all other errors.
#[derive(Fail, Debug)]
pub enum Error {
    /// Font loading error
    #[fail(display = "Font decoding error")]
    FontError(FontError),
    #[fail(display = "Texture creation error")]
    TextureError(failure::Error)
}

impl From<FontError> for Error {
    fn from(e: FontError) -> Error { Error::FontError(e) }
}

impl From<failure::Error> for Error {
    fn from(e: failure::Error) -> Error { Error::TextureError(e) }
}

/// Text renderer.
pub struct Renderer<D: Device> {
    font_bitmap: BitmapFont,
    texture: D::Texture,
    vertex_data: Vec<TexturedY8Vertex>,
}

/// Text renderer builder. Allows to set rendering options using builder
/// pattern.
///
/// # Examples
///
/// ```ignore
/// let mut file = File::open(font_path).unwrap();
/// let mut buffer = Vec::new();
/// file.read_to_end(&mut buffer);
///
/// let mut text = gfx_text::RendererBuilder::new(factory)
///     .with_size(25)
///     .with_font_data(data)
///     .build()
///     .unwrap();
/// ```
pub struct RendererBuilder<'r> {
    font_size: u8,
    font_data: Option<&'r [u8]>,
}

impl<'r> RendererBuilder<'r> {
    /// Create a new text renderer builder.
    pub fn new() -> Self {
        // Default renderer settings.
        RendererBuilder {
            font_size: DEFAULT_FONT_SIZE,
            font_data: DEFAULT_FONT_DATA,
        }
    }

    /// Specify custom size.
    pub fn with_size(mut self, size: u8) -> Self {
        self.font_size = size;
        self
    }

    /// Pass raw font data.
    pub fn with_font_data(mut self, data: &'r [u8]) -> Self {
        self.font_data = Some(data);
        self
    }

    /// Build a new text renderer instance using current settings.
    pub fn build<D: Device>(self, device: &mut D) -> Result<Renderer<D>, Error> {
        // Initialize bitmap font.
        // TODO(Kagami): Outline!
        // TODO(Kagami): More granulated font settings, e.g. antialiasing,
        // hinting, kerning, etc.
        let font_bitmap = match self.font_data {
            Some(data) => BitmapFont::from_bytes(data, self.font_size, None),
            None => Err(FontError::NoFont),
        }?;
        let texture = device.create_texture(Some(font_bitmap.get_image()),
            font_bitmap.get_width(),
            font_bitmap.get_height(),
            ColorFormat::Y8,
            false)?;

        Ok(Renderer {
            font_bitmap,
            texture,
            vertex_data: Vec::new(),
        })
    }
}

impl<D: Device> Renderer<D> {
    /// Add some text to the current draw scene relative to the top left corner
    /// of the screen using pixel coordinates.
    pub fn add(&mut self, text: &str, pos: [i32; 2], color: [f32; 4]) {
        // `Result` is used here as an `Either` analogue.
        let (mut x, y) = (pos[0] as f32, pos[1] as f32);
        for ch in text.chars() {
            let ch_info = match self.font_bitmap.find_char(ch) {
                Some(info) => info,
                // Skip unknown chars from text string. Probably it would be
                // better to place some "?" mark instead but it may not exist
                // in the font too.
                None => continue,
            };
            let x_offset = x + ch_info.x_offset as f32;
            let y_offset = y + ch_info.y_offset as f32;
            let tex = ch_info.tex;

            // Top-left point, index + 0.
            let vert0 = TexturedY8Vertex::new([x_offset, y_offset],
                [tex[0], tex[1]], color);
            // Bottom-left point, index + 1.
            let vert1 = TexturedY8Vertex::new([x_offset, y_offset + ch_info.height as f32],
                [tex[0], tex[1] + ch_info.tex_height], color);
            // Bottom-right point, index + 2.
            let vert2 = TexturedY8Vertex::new([x_offset + ch_info.width as f32, y_offset + ch_info.height as f32],
                [tex[0] + ch_info.tex_width, tex[1] + ch_info.tex_height], color);
            // Top-right point, index + 3.
            let vert3 = TexturedY8Vertex::new([x_offset + ch_info.width as f32, y_offset],
                [tex[0] + ch_info.tex_width, tex[1]], color);

            // Top-left triangle.
            // 0--3
            // | /
            // |/
            // 1
            self.vertex_data.push(vert0);
            self.vertex_data.push(vert1.clone());
            self.vertex_data.push(vert3.clone());
            // Bottom-right triangle.
            //    3
            //   /|
            //  / |
            // 1--2
            self.vertex_data.push(vert3);
            self.vertex_data.push(vert1);
            self.vertex_data.push(vert2);

            x += ch_info.x_advance as f32;
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
        transform: UnknownToDeviceTransform
    ) -> Result<(), Error> {
        device.triangles_textured_y8(target, &self.texture, false, &self.vertex_data, transform);
        self.vertex_data.clear();
        Ok(())
    }

    /// Get the bounding box size of a string as rendered by this font.
    pub fn measure(&self, text: &str) -> (i32, i32) {
        let mut width = 0;
        let mut last_char = None;

        for ch in text.chars() {
            let ch_info = match self.font_bitmap.find_char(ch) {
                Some(info) => info,
                None => continue,
            };
            last_char = Some(ch_info);

            width += ch_info.x_advance;
        }

        match last_char {
            Some(info) => width += info.x_offset + info.width - info.x_advance,
            None => (),
        }

        (width, self.font_bitmap.get_font_height() as i32)
    }
}
