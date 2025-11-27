#[derive(Clone)]
pub struct Fonts;

impl drawing_api::Fonts for Fonts {
    fn register_font(
        &self,
        font_data: &[u8],
        family_name_alias: Option<&str>,
    ) -> Result<(), &'static str> {
        todo!()
    }
}

impl Default for Fonts {
    fn default() -> Self {
        Self {}
    }
}
