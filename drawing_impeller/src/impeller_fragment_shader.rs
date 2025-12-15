use std::borrow::Cow;

#[derive(Clone)]
pub struct ImpellerFragmentShader {
    fragment_program: impellers::FragmentProgram,
}

impl drawing_api::FragmentShader for ImpellerFragmentShader {
    unsafe fn new(program: Cow<'static, [u8]>) -> Result<Self, &'static str> {
        unsafe {
            Ok(ImpellerFragmentShader {
                fragment_program: impellers::FragmentProgram::new(program)
                    .ok_or("Cannot create impeller fragment program")?,
            })
        }
    }
}
