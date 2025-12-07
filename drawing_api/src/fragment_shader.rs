/// Represents a fragment shader program.
/// Reference counted, thread safe, immutable object.
pub trait FragmentShader: Sized + Sync + Send + Clone {}
