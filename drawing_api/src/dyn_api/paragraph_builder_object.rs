use std::any::Any;

use crate::{ParagraphBuilder, ParagraphStyle};

use super::{PaintObject, ParagraphObject};

pub trait ParagraphBuilderObject {
    fn push_style(&mut self, style: &ParagraphStyle<Box<dyn PaintObject>>);

    fn pop_style(&mut self);

    fn add_text(&mut self, text: &str);

    fn build(self) -> Result<Box<dyn ParagraphObject>, &'static str>;
}

impl<B: ParagraphBuilder> ParagraphBuilderObject for B {
    fn push_style(&mut self, style: &ParagraphStyle<Box<dyn PaintObject>>) {
        let foreground = style
            .foreground
            .as_ref()
            .map(|p| (p as &dyn Any).downcast_ref::<B::Paint>().unwrap().clone());
        let background = style
            .background
            .as_ref()
            .map(|p| (p as &dyn Any).downcast_ref::<B::Paint>().unwrap().clone());
        let style = ParagraphStyle::<B::Paint> {
            foreground,
            background,
            weight: style.weight,
            style: style.style,
            family: style.family.clone(),
            size: style.size,
            height_factor: style.height_factor,
            text_alignment: style.text_alignment,
            text_direction: style.text_direction,
            text_decoration: style.text_decoration.clone(),
            max_lines: style.max_lines,
            ellipsis: style.ellipsis.clone(),
            locale: style.locale.clone(),
        };
        self.push_style(style);
    }

    fn pop_style(&mut self) {
        self.pop_style();
    }

    fn add_text(&mut self, text: &str) {
        self.add_text(text);
    }

    fn build(self) -> Result<Box<dyn ParagraphObject>, &'static str> {
        Ok(Box::new(self.build()?))
    }
}
