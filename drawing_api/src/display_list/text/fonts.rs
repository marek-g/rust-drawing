/// Reference counted object.
pub trait Fonts: Clone + Default {
    fn register_font(
        &mut self,
        font_data: &[u8],
        family_name_alias: Option<&str>,
    ) -> Result<(), &'static str>;
}
