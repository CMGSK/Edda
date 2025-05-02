use std::fmt::Write;

use super::{
    style::{Style, UnderlineStyle},
    text::StyledText,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParagraphModifyError {
    #[error("Chunk to modify not found in paragraph: '{0}'")]
    ChunkNotFound(String),
}

pub enum ApplicableStyles {
    Bold,
    Italic,
    Underline(Option<UnderlineStyle>),
    Size(u8),
    Font(String),
    Color(String),
    Highlight(Option<String>),
}

/// Collection of text chunks with its own styles
#[derive(Debug, PartialEq)]
pub struct StyledParagraph {
    pub raw: Vec<StyledText>,
}

impl StyledParagraph {
    pub fn new() -> Self {
        StyledParagraph { raw: Vec::new() }
    }

    pub fn insert(&mut self, idx: usize, new: StyledText) {
        self.raw.insert(idx, new);
    }

    pub fn add(&mut self, new: StyledText) {
        self.raw.push(new);
    }

    //TODO: This is hideous
    pub fn modify(&mut self, style: Style, chunk: &str) -> Result<(), ParagraphModifyError> {
        let (idx, dif) = self
            .raw
            .iter()
            .enumerate()
            .find(|(_n, st)| st.text.contains(chunk))
            .map(|(n, st)| (n, st.clone()))
            .ok_or_else(|| ParagraphModifyError::ChunkNotFound(chunk.to_string()))?;

        let start_offset = dif
            .text
            .find(chunk)
            .ok_or_else(|| ParagraphModifyError::ChunkNotFound(chunk.to_string()))?;
        let end_offset = start_offset + chunk.len();

        let mut current_idx = idx;

        self.raw.remove(idx);

        let prepend_text = &dif.text[..start_offset];
        if !prepend_text.is_empty() {
            self.raw.insert(
                current_idx,
                StyledText::new(prepend_text.into(), dif.style.clone()),
            );
            current_idx += 1;
        }

        let new_st = StyledText::new(chunk.into(), style);
        self.raw.insert(current_idx, new_st);
        current_idx += 1;

        let append_text = &dif.text[end_offset..];
        if !append_text.is_empty() {
            self.raw
                .insert(current_idx, StyledText::new(append_text.into(), dif.style));
        }

        Ok(())
    }

    #[allow(dead_code)]
    fn parse_as_raw_tagged_text(&self) -> String {
        let mut buffer = String::new();
        for x in &self.raw {
            let _ = write!(buffer, "{}", x.apply_style_tagging());
        }
        buffer
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stylemgr::style::{Style, UnderlineStyle};
    use crate::stylemgr::text::StyledText;

    #[test]
    fn test_paragraph_new() {
        let p = StyledParagraph::new();
        assert!(p.raw.is_empty());
    }

    #[test]
    fn test_paragraph_add() {
        let mut p = StyledParagraph::new();
        let st1 = StyledText::new("Hello ".to_string(), Style::new());
        let st2 = StyledText::new("World".to_string(), Style::new().switch_bold());
        p.add(st1);
        p.add(st2);
        assert_eq!(p.raw.len(), 2);
        assert_eq!(p.raw[0].text, "Hello ");
        assert_eq!(p.raw[1].text, "World");
        assert!(!p.raw[0].style.bold());
        assert!(p.raw[1].style.bold());
    }

    #[test]
    fn test_paragraph_insert() {
        let mut p = StyledParagraph::new();
        let st1 = StyledText::new("First".to_string(), Style::new());
        let st2 = StyledText::new("Third".to_string(), Style::new());
        let st_ins = StyledText::new("Second".to_string(), Style::new().switch_italic());
        p.add(st1);
        p.add(st2);
        p.insert(1, st_ins); // Insert at index 1

        assert_eq!(p.raw.len(), 3);
        assert_eq!(p.raw[0].text, "First");
        assert_eq!(p.raw[1].text, "Second");
        assert_eq!(p.raw[2].text, "Third");
        assert!(p.raw[1].style.italic());
    }

    #[test]
    fn test_paragraph_modify_simple() {
        let mut p = StyledParagraph::new();
        let st1 = StyledText::new("This is a test.".to_string(), Style::new());
        p.add(st1);

        let bold_style = Style::new().switch_bold();
        let result = p.modify(bold_style, "is a");

        assert!(result.is_ok());
        assert_eq!(p.raw.len(), 3);
        assert_eq!(p.raw[0].text, "This ");
        assert!(!p.raw[0].style.bold());
        assert_eq!(p.raw[1].text, "is a");
        assert!(p.raw[1].style.bold());
        assert_eq!(p.raw[2].text, " test.");
        assert!(!p.raw[2].style.bold());
    }

    #[test]
    fn test_paragraph_modify_full_chunk() {
        let mut p = StyledParagraph::new();
        let st1 = StyledText::new("Part1 ".to_string(), Style::new());
        let st2 = StyledText::new("ModifyMe".to_string(), Style::new());
        let st3 = StyledText::new(" Part3".to_string(), Style::new());
        p.add(st1);
        p.add(st2);
        p.add(st3);

        let italic_style = Style::new().switch_italic();
        let result = p.modify(italic_style, "ModifyMe");

        assert!(result.is_ok());
        assert_eq!(p.raw.len(), 3); // Should replace st2, not split it
        assert_eq!(p.raw[0].text, "Part1 ");
        assert!(!p.raw[0].style.italic());
        assert_eq!(p.raw[1].text, "ModifyMe");
        assert!(p.raw[1].style.italic());
        assert_eq!(p.raw[2].text, " Part3");
        assert!(!p.raw[2].style.italic());
    }

    #[test]
    fn test_paragraph_modify_start() {
        let mut p = StyledParagraph::new();
        let st1 = StyledText::new("Prefix suffix".to_string(), Style::new());
        p.add(st1);

        let bold_style = Style::new().switch_bold();
        let result = p.modify(bold_style, "Prefix");

        assert!(result.is_ok());
        assert_eq!(p.raw.len(), 2);
        assert_eq!(p.raw[0].text, "Prefix");
        assert!(p.raw[0].style.bold());
        assert_eq!(p.raw[1].text, " suffix");
        assert!(!p.raw[1].style.bold());
    }

    #[test]
    fn test_paragraph_modify_end() {
        let mut p = StyledParagraph::new();
        let st1 = StyledText::new("Prefix suffix".to_string(), Style::new());
        p.add(st1);

        let bold_style = Style::new().switch_bold();
        let result = p.modify(bold_style, "suffix");

        assert!(result.is_ok());
        assert_eq!(p.raw.len(), 2);
        assert_eq!(p.raw[0].text, "Prefix ");
        assert!(!p.raw[0].style.bold());
        assert_eq!(p.raw[1].text, "suffix");
        assert!(p.raw[1].style.bold());
    }

    #[test]
    fn test_paragraph_modify_chunk_not_found() {
        let mut p = StyledParagraph::new();
        let st1 = StyledText::new("Some text here.".to_string(), Style::new());
        p.add(st1);

        let bold_style = Style::new().switch_bold();
        let result = p.modify(bold_style, "nonexistent");

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ParagraphModifyError::ChunkNotFound(_)
        ));
        assert_eq!(p.raw.len(), 1); // Ensure original state is preserved
        assert_eq!(p.raw[0].text, "Some text here.");
    }

    #[test]
    fn test_parse_as_raw_tagged_text() {
        let mut p = StyledParagraph::new();
        let style1 = Style::new();
        let style2 = Style::new()
            .switch_bold()
            .set_underline(Some(UnderlineStyle::Double));
        let st1 = StyledText::new("Plain ".to_string(), style1.clone());
        let st2 = StyledText::new("BoldUnderline".to_string(), style2.clone());
        p.add(st1);
        p.add(st2);

        // Expected format depends on StyledText::apply_style_tagging
        let tag1 = format!("{}", style1);
        let tag2 = format!("{}", style2);
        let expected = format!(
            "[[{0}]]Plain [[/{0}]][[{1}]]BoldUnderline[[/{1}]]",
            tag1, tag2
        );

        assert_eq!(p.parse_as_raw_tagged_text(), expected);
    }
}
