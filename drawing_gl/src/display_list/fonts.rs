use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::generic::{
    device::Device,
    texture_font::{Font, TextureFont},
};

pub(crate) struct FontsData<D: Device> {
    pub fonts: HashMap<String, TextureFont<D>>,
}

pub struct Fonts<D: Device> {
    pub(crate) data: Rc<RefCell<FontsData<D>>>,
}

impl<D: Device> Clone for Fonts<D> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
        }
    }
}

impl<D: Device> Fonts<D> {
    pub fn new() -> Self {
        Self {
            data: Rc::new(RefCell::new(FontsData {
                fonts: HashMap::new(),
            })),
        }
    }
}

impl<D: Device> drawing_api::Fonts for Fonts<D> {
    fn register_font(
        &self,
        font_data: &[u8],
        family_name_alias: Option<&str>,
    ) -> Result<(), &'static str> {
        let font = TextureFont::<D>::create(Vec::from(font_data))?;
        let family_name = family_name_alias.unwrap_or("default");
        self.data
            .borrow_mut()
            .fonts
            .insert(family_name.into(), font);
        Ok(())
    }
}
