//! A library for drawing text for gfx-rs graphics API.
//! Uses freetype-rs underneath to former the font bitmap texture and collect
//! information about face glyphs.
//!
//! # Examples
//!
//! Basic usage:
//!
//! ```ignore
//! // Initialize text renderer.
//! let mut text = gfx_text::new(factory).build().unwrap();
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
//! text.draw(&mut stream);
//! ```

#![deny(missing_docs)]

extern crate freetype;

use units::UnknownToDeviceTransform;
use backend::TexturedY8Vertex;
use color::ColorFormat;
use backend::Backend;
use texture_font::font::BitmapFont;
pub use texture_font::font::FontError;

const DEFAULT_FONT_SIZE: u8 = 16;
const DEFAULT_BUFFER_SIZE: usize = 128;
const DEFAULT_OUTLINE_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

#[cfg(feature = "include-font")]
const DEFAULT_FONT_DATA: Option<&'static [u8]> =
    Some(include_bytes!("../assets/NotoSans-Regular.ttf"));
#[cfg(not(feature = "include-font"))]
const DEFAULT_FONT_DATA: Option<&'static [u8]> =
    None;

/// General error type returned by the library. Wraps all other errors.
#[derive(Debug)]
pub enum Error {
    /// Font loading error
    FontError(FontError),
}

/// An anchor aligns text horizontally to its given x position.
#[derive(PartialEq)]
pub enum HorizontalAnchor {
    /// Anchor the left edge of the text
    Left,
    /// Anchor the horizontal mid-point of the text
    Center,
    /// Anchor the right edge of the text
    Right,
}

/// An anchor aligns text vertically to its given y position.
#[derive(PartialEq)]
pub enum VerticalAnchor {
    /// Anchor the top edge of the text
    Top,
    /// Anchor the vertical mid-point of the text
    Center,
    /// Anchor the bottom edge of the text
    Bottom,
}

impl From<FontError> for Error {
    fn from(e: FontError) -> Error { Error::FontError(e) }
}

/// Text renderer.
pub struct Renderer<B: Backend> {
    font_bitmap: BitmapFont,
    texture: B::Texture,
    vertex_data: Vec<TexturedY8Vertex>,
}

/// Text renderer builder. Allows to set rendering options using builder
/// pattern.
///
/// # Examples
///
/// ```ignore
/// let mut text = gfx_text::RendererBuilder::new(factory)
///     .with_size(25)
///     .with_font("/path/to/font.ttf")
///     .with_chars(&['a', 'b', 'c'])
///     .build()
///     .unwrap();
/// ```
pub struct RendererBuilder<'r> {
    font_size: u8,
    // NOTE(Kagami): Better to use `P: AsRef<OsStr>` but since we store path in
    // the intermediate builder structure, Rust will unable to infer type
    // without manual annotation which is much worse. Anyway, it's possible to
    // just pass raw bytes.
    font_path: Option<&'r str>,
    font_data: Option<&'r [u8]>,
    outline_width: Option<u8>,
    outline_color: [f32; 4],
    buffer_size: usize,
    chars: Option<&'r [char]>,
}

impl<'r> RendererBuilder<'r> {
    /// Create a new text renderer builder.
    pub fn new() -> Self {
        // Default renderer settings.
        RendererBuilder {
            font_size: DEFAULT_FONT_SIZE,
            font_path: None,  // Default font will be used
            font_data: DEFAULT_FONT_DATA,
            outline_width: None,  // No outline by default
            outline_color: DEFAULT_OUTLINE_COLOR,
            buffer_size: DEFAULT_BUFFER_SIZE,
            chars: None,  // Place all available font chars into texture
        }
    }

    /// Specify custom size.
    pub fn with_size(mut self, size: u8) -> Self {
        self.font_size = size;
        self
    }

    /// Specify custom font by path.
    pub fn with_font(mut self, path: &'r str) -> Self {
        self.font_path = Some(path);
        self
    }

    /// Pass raw font data.
    pub fn with_font_data(mut self, data: &'r [u8]) -> Self {
        self.font_data = Some(data);
        self
    }

    /// Specify outline width and color.
    /// **Not implemented yet.**
    pub fn with_outline(mut self, width: u8, color: [f32; 4]) -> Self {
        self.outline_width = Some(width);
        self.outline_color = color;
        self
    }

    /// Specify custom initial buffer size.
    pub fn with_buffer_size(mut self, size: usize) -> Self {
        self.buffer_size = size;
        self
    }

    /// Make available only provided characters in font texture instead of
    /// loading all existing from the font face.
    pub fn with_chars(mut self, chars: &'r [char]) -> Self {
        self.chars = Some(chars);
        self
    }

    /// Build a new text renderer instance using current settings.
    pub fn build<B: Backend>(mut self, backend: &mut B) -> Result<Renderer<B>, Error> {
        // Initialize bitmap font.
        // TODO(Kagami): Outline!
        // TODO(Kagami): More granulated font settings, e.g. antialiasing,
        // hinting, kerning, etc.
        let font_bitmap = try!(match self.font_path {
            Some(path) =>
                BitmapFont::from_path(path, self.font_size, self.chars),
            None => match self.font_data {
                Some(data) => BitmapFont::from_bytes(data, self.font_size, self.chars),
                None => Err(FontError::NoFont),
            },
        });
        let texture = backend.create_texture(font_bitmap.get_image(),
            font_bitmap.get_width(),
            font_bitmap.get_height(),
            ColorFormat::Y8,
            false);

        Ok(Renderer {
            font_bitmap,
            texture,
            vertex_data: Vec::new(),
        })
    }
}

impl<B: Backend> Renderer<B> {
    /// Add some text to the current draw scene relative to the top left corner
    /// of the screen using pixel coordinates.
    pub fn add(&mut self, text: &str, pos: [i32; 2], color: [f32; 4]) {
        self.add_generic(text, Ok(pos), color)
    }

    /// Add text to the draw scene by anchoring an edge or mid-point to a
    /// position defined in screen pixel coordinates.
    pub fn add_anchored(&mut self, text: &str, pos: [i32; 2], horizontal: HorizontalAnchor, vertical: VerticalAnchor, color: [f32; 4]) {
        if horizontal == HorizontalAnchor::Left && vertical == VerticalAnchor::Top {
            self.add_generic(text, Ok(pos), color);
            return
        }

        let (width, height) = self.measure(text);
        let x = match horizontal {
            HorizontalAnchor::Left => pos[0],
            HorizontalAnchor::Center => pos[0] - width / 2,
            HorizontalAnchor::Right => pos[0] - width,
        };
        let y = match vertical {
            VerticalAnchor::Top => pos[1],
            VerticalAnchor::Center => pos[1] - height / 2,
            VerticalAnchor::Bottom => pos[1] - height,
        };

        self.add_generic(text, Ok([x, y]), color)
    }

    /// Add some text to the draw scene using absolute world coordinates.
    pub fn add_at(&mut self, text: &str, pos: [f32; 3], color: [f32; 4]) {
        self.add_generic(text, Err(pos), color)
    }

    fn add_generic(&mut self, text: &str, pos: Result<[i32; 2], [f32; 3]>, color: [f32; 4]) {
        // `Result` is used here as an `Either` analogue.
        let (screen_pos, world_pos, screen_rel) = match pos {
            Ok(screen_pos) => (screen_pos, [0.0, 0.0, 0.0], 1),
            Err(world_pos) => ([0, 0], world_pos, 0),
        };
        let (mut x, y) = (screen_pos[0] as f32, screen_pos[1] as f32);
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
            let index = self.vertex_data.len() as u32;

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
    /// text.draw_at(&mut backend, &color_output, camera_projection).unwrap();
    /// ```
    pub fn draw_at(
        &mut self,
        backend: &mut B,
        target: &B::RenderTarget,
        transform: UnknownToDeviceTransform
    ) -> Result<(), Error> {
        backend.triangles_textured_y8(target, &self.texture, false, &self.vertex_data, transform);
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

// Some missing helpers.

fn grow_buffer_size(mut current_size: usize, desired_size: usize) -> usize {
    if current_size < 1 {
        current_size = 1;
    }
    while current_size < desired_size {
        current_size *= 2;
    }
    current_size
}
