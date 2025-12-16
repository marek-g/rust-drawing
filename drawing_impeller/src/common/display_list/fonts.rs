use std::{borrow::Cow, cell::RefCell, rc::Rc};

#[derive(Clone)]
pub struct Fonts {
    pub(crate) typography_context: Rc<RefCell<impellers::TypographyContext>>,
}

impl Default for Fonts {
    fn default() -> Self {
        Self {
            typography_context: Rc::new(RefCell::new(impellers::TypographyContext::default())),
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
            .borrow_mut()
            .register_font(font_data, family_name_alias)
    }
}
