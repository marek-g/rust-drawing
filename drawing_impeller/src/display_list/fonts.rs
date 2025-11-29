#[derive(Clone)]
pub struct Fonts {
    pub(crate) typography_context: impellers::TypographyContext,

    // font data must be hold in the memory, because impeller
    // access it each time paragraph builder access new glyph
    fonts_data: Vec<Vec<u8>>,
}

impl Default for Fonts {
    fn default() -> Self {
        Self {
            typography_context: impellers::TypographyContext::default(),
            fonts_data: Vec::new(),
        }
    }
}

impl drawing_api::Fonts for Fonts {
    fn register_font(
        &mut self,
        font_data: &[u8],
        family_name_alias: Option<&str>,
    ) -> Result<(), &'static str> {
        let data = Vec::from(font_data);
        self.fonts_data.push(data);
        self.typography_context
            .register_font(&self.fonts_data.last().unwrap(), family_name_alias)
    }
}
