use std::borrow::Cow;

/// Represents a fragment shader program.
/// Reference counted, thread safe, immutable object.
pub trait FragmentShader: Sized + Sync + Send + Clone {
    unsafe fn new(program: Cow<'static, [u8]>) -> Result<Self, &'static str>;
}
