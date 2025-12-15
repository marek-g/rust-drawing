#[derive(Clone)]
pub struct GlFragmentShader {}

impl drawing_api::FragmentShader for GlFragmentShader {
    unsafe fn new(program: std::borrow::Cow<'static, [u8]>) -> Result<Self, &'static str> {
        todo!()
    }
}
