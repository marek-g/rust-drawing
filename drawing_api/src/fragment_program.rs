use std::borrow::Cow;

/// Represents a fragment program.
/// Reference counted, thread safe, immutable object.
pub trait FragmentProgram: Sized + Sync + Send + Clone {
    unsafe fn new(program: Cow<'static, [u8]>) -> Result<Self, &'static str>;
}
