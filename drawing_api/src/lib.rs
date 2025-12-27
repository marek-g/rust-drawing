mod common;
use std::ops::Deref;

pub use common::*;

mod gl;
pub use gl::*;

mod vulkan;
pub use vulkan::*;

pub mod dyn_api;

/// Represents either an owned or borrowed T.
/// Useful to pass `T` or `&'a T` to a function when used as `Into<OptRef<'a, T>>`.
///
/// It is defined in this crate, so the From trait can be implemented for foreigin types.
pub enum OptRef<'a, T> {
    Borrowed(&'a T),
    Owned(T),
}

impl<T> Deref for OptRef<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        match self {
            Self::Owned(v) => v,
            Self::Borrowed(v) => *v,
        }
    }
}

impl<'a, T: Clone> OptRef<'a, T> {
    pub fn to_owned(self) -> T {
        match self {
            OptRef::Borrowed(v) => v.clone(),
            OptRef::Owned(v) => v,
        }
    }
}

/// Represents owned T.
/// Useful to pass `T` to a function when used as `Into<Owned<T>>`.
///
/// It is defined in this crate, so the From trait can be implemented for foreigin types.
pub struct Owned<T>(T);
