use std::borrow::Cow;

/// Reference counted object.
pub trait Fonts: Clone + Default + 'static {
    fn register_font(
        &mut self,
        font_data: Cow<'static, [u8]>,
        family_name_alias: Option<&str>,
    ) -> Result<(), &'static str>;
}
