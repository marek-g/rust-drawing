extern crate euclid;
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate failure;

pub type Result<T> = std::result::Result<T, failure::Error>;

pub mod backend;
pub mod clipping;
pub mod color;
pub mod composite_operation_state;
pub mod font;
pub mod paint;
pub mod path;
pub mod primitive;
pub mod primitive_extensions;
pub mod renderer;
pub mod resources;
pub mod transformation;
pub mod units;

mod texture_font;
pub use texture_font::TextureFont;
