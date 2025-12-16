#[derive(Clone)]
pub struct GlFragmentProgram {}

impl drawing_api::FragmentProgram for GlFragmentProgram {
    unsafe fn new(program: std::borrow::Cow<'static, [u8]>) -> Result<Self, &'static str> {
        todo!()
    }
}
