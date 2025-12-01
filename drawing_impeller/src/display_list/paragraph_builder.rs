use crate::ImpellerTexture;

use super::convert_paragraph_style;

pub struct ParagraphBuilder {
    pub(crate) paragraph_builder: impellers::ParagraphBuilder,
}

impl drawing_api::ParagraphBuilder for ParagraphBuilder {
    type Paragraph = crate::Paragraph;
    type Paint = crate::Paint;
    type Fonts = crate::Fonts;
    type Texture = crate::ImpellerTexture;

    fn new(fonts: &crate::Fonts) -> Result<Self, &'static str> {
        Ok(ParagraphBuilder {
            paragraph_builder: impellers::ParagraphBuilder::new(&fonts.typography_context)
                .ok_or("Couldn't create impeller ParagraphBuilder")?,
        })
    }

    fn push_style(&mut self, style: drawing_api::ParagraphStyle<ImpellerTexture, crate::Paint>) {
        println!("----------------- Style: size: {}", style.size);
        let paragraph_style = convert_paragraph_style(&style);
        self.paragraph_builder.push_style(&paragraph_style);
    }

    fn pop_style(&mut self) {
        self.paragraph_builder.pop_style();
    }

    fn add_text(&mut self, text: &str) {
        println!("----------------- Paragraph: {}", text);
        self.paragraph_builder.add_text(text);
    }

    fn build(mut self) -> Result<Self::Paragraph, &'static str> {
        let paragraph = self
            .paragraph_builder
            .build(600.0f32)
            .ok_or("Impeller couldn't build the paragraph")?;

        println!(
            "alphabetic_baseline: {}",
            paragraph.get_alphabetic_baseline()
        );
        println!("height: {}", paragraph.get_height());
        println!(
            "ideographic_baseline: {}",
            paragraph.get_ideographic_baseline()
        );
        println!("line_count: {}", paragraph.get_line_count());
        println!("longest_line_width: {}", paragraph.get_longest_line_width());
        println!(
            "max_intrinsic_width: {}",
            paragraph.get_max_intrinsic_width()
        );
        println!("max_width: {}", paragraph.get_max_width());
        println!(
            "min_intrinsic_width: {}",
            paragraph.get_min_intrinsic_width()
        );

        println!("     GLYPH 0:");
        let glyph_info = paragraph
            .create_glyph_info_at_code_unit_index_utf16(0)
            .unwrap();
        println!(
            "  grapheme_cluser_bounds: {:?}",
            glyph_info.get_grapheme_cluster_bounds()
        );
        println!(
            "  code_unit_range_begin: {}",
            glyph_info.get_grapheme_cluster_code_unit_range_begin_utf16()
        );
        println!(
            "  code_unit_range_end: {}",
            glyph_info.get_grapheme_cluster_code_unit_range_end_utf16()
        );
        println!("  text direction: {:?}", glyph_info.get_text_direction());
        println!("  is ellipsis: {:?}", glyph_info.is_ellipsis());

        println!("     Line metrics for line 0:");
        let line_metrics = paragraph.get_line_metrics().unwrap();
        println!("  width: {:?}", line_metrics.get_width(0));
        println!("  height: {:?}", line_metrics.get_height(0));
        println!("  height: {:?}", line_metrics.get_ascent(0));

        println!("     Word boundary for code unit 0:");
        let word_boundary = paragraph.get_word_boundary_utf16(0);
        println!("   range: {:?}", word_boundary);

        //println!("height: {}", paragraph.get_word_boundary_utf16(code_unit_index));
        //println!("line_metrics: {:?}", paragraph..get_line_metrics());

        Ok(crate::Paragraph { paragraph })
    }
}
