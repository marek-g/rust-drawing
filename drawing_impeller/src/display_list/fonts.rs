use std::borrow::Cow;

#[derive(Clone)]
pub struct Fonts {
    pub(crate) typography_context: impellers::TypographyContext,
}

impl Default for Fonts {
    fn default() -> Self {
        Self {
            typography_context: impellers::TypographyContext::default(),
        }
    }
}

impl drawing_api::Fonts for Fonts {
    fn register_font(
        &mut self,
        font_data: Cow<'static, [u8]>,
        family_name_alias: Option<&str>,
    ) -> Result<(), &'static str> {
        self.typography_context
            .register_font(font_data, family_name_alias)
    }
}
