use std::any::Any;

use crate::FragmentProgram;

pub trait FragmentProgramObject: Any {}

impl<F: FragmentProgram> FragmentProgramObject for F {}
