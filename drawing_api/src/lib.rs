mod common;
pub use common::*;

mod gl;
pub use gl::*;

mod vulkan;
pub use vulkan::*;

/// Represents either an owned or borrowed T.
/// Useful to pass `T` or `&'a T` to a function when used as `Into<OptRef<'a, T>>`.
pub enum OptRef<'a, T> {
    Borrowed(&'a T),
    Owned(T),
}
