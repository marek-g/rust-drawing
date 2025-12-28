mod common;
mod gl;
mod vulkan;

pub use crate::common::*;
pub use crate::gl::*;
pub use crate::vulkan::*;

pub mod prelude {
    pub use crate::common::*;
    pub use crate::gl::*;
    pub use crate::vulkan::*;
    pub use ::euclid::rect;
}

pub mod dyn_api;

pub mod euclid {
    pub use ::euclid::*;
}

pub mod smart_pointers {
    use std::ops::Deref;

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
    pub struct Owned<T>(pub T);
}
