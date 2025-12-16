use std::borrow::Cow;

#[derive(Clone)]
pub struct ImpellerFragmentProgram {
    pub(crate) fragment_program: impellers::FragmentProgram,
}

impl drawing_api::FragmentProgram for ImpellerFragmentProgram {
    unsafe fn new(program: Cow<'static, [u8]>) -> Result<Self, &'static str> {
        unsafe {
            Ok(ImpellerFragmentProgram {
                fragment_program: impellers::FragmentProgram::new(program)
                    .ok_or("Cannot create impeller fragment program")?,
            })
        }
    }
}
