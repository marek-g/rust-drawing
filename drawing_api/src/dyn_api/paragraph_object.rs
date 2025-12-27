use crate::Paragraph;

pub trait ParagraphObject {}

impl<P: Paragraph> ParagraphObject for P {}
