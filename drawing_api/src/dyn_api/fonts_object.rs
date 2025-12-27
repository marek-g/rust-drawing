use std::{any::Any, borrow::Cow};

use crate::Fonts;

pub trait FontsObject: Any {
    fn register_font(
        &mut self,
        font_data: Cow<'static, [u8]>,
        family_name_alias: Option<&str>,
    ) -> Result<(), &'static str>;
}

impl<F: Fonts> FontsObject for F {
    fn register_font(
        &mut self,
        font_data: Cow<'static, [u8]>,
        family_name_alias: Option<&str>,
    ) -> Result<(), &'static str> {
        self.register_font(font_data, family_name_alias)
    }
}
