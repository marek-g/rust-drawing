use crate::DisplayList;

pub trait DisplayListObject {}

impl<D: DisplayList> DisplayListObject for D {}
