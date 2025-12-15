use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};

use crate::generic::{
    device::Device,
    texture_font::{Font, TextureFont},
};

pub(crate) struct FontsData<D: Device> {
    pub fonts: HashMap<String, TextureFont<D>>,
}

pub struct Fonts<D: Device> {
    pub(crate) data: Arc<Mutex<FontsData<D>>>,
}

impl<D: Device> Clone for Fonts<D> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
        }
    }
}

impl<D: Device> Debug for Fonts<D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Fonts")
            .field("data", &self.data.lock().unwrap().fonts.keys())
            .finish()
    }
}

impl<D: Device> Default for Fonts<D> {
    fn default() -> Self {
        Self {
            data: Arc::new(Mutex::new(FontsData {
                fonts: HashMap::new(),
            })),
        }
    }
}

impl<D: Device> drawing_api::Fonts for Fonts<D> {
    fn register_font(
        &mut self,
        font_data: Cow<'static, [u8]>,
        family_name_alias: Option<&str>,
    ) -> Result<(), &'static str> {
        let font = TextureFont::<D>::create(Vec::from(font_data))?;
        let family_name = family_name_alias.unwrap_or("default");
        self.data
            .lock()
            .unwrap()
            .fonts
            .insert(family_name.into(), font);
        Ok(())
    }
}
