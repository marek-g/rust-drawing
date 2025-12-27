use std::any::Any;

use crate::ColorSourceFragment;

pub trait ColorSourceFragmentObject: Any {}

impl<F: ColorSourceFragment> ColorSourceFragmentObject for F {}
