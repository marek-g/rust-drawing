use std::{cell::RefCell, rc::Rc};

pub static mut ttx: Option<impellers::TypographyContext> = None;

#[derive(Clone)]
pub struct Fonts {
    //pub(crate) typography_context: Rc<RefCell<impellers::TypographyContext>>,
}

impl Default for Fonts {
    fn default() -> Self {
        unsafe {
            if let None = ttx {
                ttx = Some(impellers::TypographyContext::default());
            }
        }

        // TODO: what?
        //let ttx = Box::leak(Box::new(impellers::TypographyContext::default()));
        //let ttx2 = impellers::TypographyContext::default();
        //let ttx2 = Box::leak(Box::new(ttx2));
        /*let ttx_clone = unsafe {
                #![allow(static_mut_refs)]
                let ttx2 = ttx.as_mut().unwrap();
                ttx2.clone()
            };
            //Box::leak(Box::new(ttx_clone));
            Self {
                //typography_context: Rc::new(RefCell::new(impellers::TypographyContext::default())),
                typography_context: Rc::new(RefCell::new(ttx_clone)),
        }*/
        Self {}
    }
}

impl drawing_api::Fonts for Fonts {
    fn register_font(
        &mut self,
        font_data: &[u8],
        family_name_alias: Option<&str>,
    ) -> Result<(), &'static str> {
        /*self.typography_context
            .borrow_mut()
        .register_font(font_data, family_name_alias)*/

        unsafe {
            #![allow(static_mut_refs)]
            let ttx2 = ttx.as_mut().unwrap();

            ttx2.register_font(font_data, family_name_alias)
        }
    }
}
