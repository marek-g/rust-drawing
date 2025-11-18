/// Reference counted object.
pub trait Fonts: Clone {
    fn register_font(
        &self,
        font_data: &[u8],
        family_name_alias: Option<&str>,
    ) -> Result<(), &'static str>;
}
