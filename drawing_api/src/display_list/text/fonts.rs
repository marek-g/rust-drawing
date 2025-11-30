/// Reference counted object.
pub trait Fonts: Clone + Default {
    fn register_font(
        &mut self,
        font_data: Box<[u8]>,
        family_name_alias: Option<&str>,
    ) -> Result<(), &'static str>;
}
