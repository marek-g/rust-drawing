use std::any::Any;

use crate::ImageFilterFragment;

pub trait ImageFilterFragmentObject: Any {}

impl<F: ImageFilterFragment> ImageFilterFragmentObject for F {}
