mod common;
pub use common::*;

mod gl;
pub use gl::*;

mod vulkan;
pub use vulkan::*;

pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L, R> Either<L, R> {}
