extern crate euclid;
#[macro_use]
extern crate failure;

pub type Result<T> = std::result::Result<T, failure::Error>;

pub mod backend;
pub mod color;
pub mod font;
pub mod primitive;
pub mod primitive_extensions;
pub mod renderer;
pub mod resources;
pub mod units;
mod utils;

mod texture_font;
pub use texture_font::TextureFont;
