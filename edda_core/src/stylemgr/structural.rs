use std::fmt::Write;

use super::{
    style::{Style, UnderlineStyle},
    text::StyledText,
};
use thiserror::Error;

/// Errors that can occur when modifying a `StyledParagraph`.
#[derive(Debug, Error, PartialEq)]
pub enum ParagraphModifyError {
    /// The specified text chunk to modify was not found within any single `StyledText`
    /// segment of the paragraph.
    #[error("Chunk to modify not found in paragraph: '{0}'")]
    ChunkNotFound(String),
    /// The chunk provided for modification was empty.
    #[error("Cannot modify paragraph with an empty chunk")]
    EmptyChunk,
}

/// Represents specific style attributes that can be applied.
///
/// Note: This enum is currently only used internally by `StyledText::change_style`.
// TODO: Consider if this enum is still the best approach or if direct Style manipulation is preferred.
#[derive(Debug, Clone, PartialEq)]
pub enum ApplicableStyles {
    /// Apply or toggle bold.
    Bold,
    /// Apply or toggle italic.
    Italic,
    /// Apply or remove underline. `None` removes, `Some(style)` applies the specified style.
    Underline(Option<UnderlineStyle>),
    /// Change the font size (in points).
    Size(u8),
    /// Change the font family name.
    Font(String),
    /// Change the font color (hex string, e.g., "#FF0000").
    Color(String),
    /// Apply or remove text highlighting. `None` removes, `Some(color)` applies the specified color (hex string).
    Highlight(Option<String>),
}

/// Represents a paragraph composed of multiple text chunks (`StyledText`),
/// each potentially having its own distinct style.
#[derive(Debug, Default, Clone, PartialEq)] // Added Default, Clone
pub struct StyledParagraph {
    /// The sequence of styled text chunks that make up the paragraph.
    pub raw: Vec<StyledText>,
}

impl StyledParagraph {
    /// Creates a new, empty `StyledParagraph`.
    #[must_use = "Creating a new paragraph does nothing unless used"]
    pub fn new() -> Self {
        // Default::default() is equivalent here due to derive
        StyledParagraph { raw: Vec::new() }
    }

    /// Inserts a `StyledText` chunk at the specified index.
    ///
    /// # Panics
    /// Panics if `idx` is greater than the number of chunks currently in the paragraph.
    pub fn insert(&mut self, idx: usize, new: StyledText) {
        self.raw.insert(idx, new);
    }

    /// Appends a `StyledText` chunk to the end of the paragraph.
    pub fn add(&mut self, new: StyledText) {
        self.raw.push(new);
    }

    /// Modifies the style of the first occurrence of a specific text `chunk` within the paragraph.
    ///
    /// This method finds the first `StyledText` segment containing the `chunk`. It then splits
    /// that segment into up to three parts: the text before the chunk (keeping original style),
    /// the chunk itself (applying the new `style`), and the text after the chunk (keeping original style).
    /// The original `StyledText` segment is replaced by these new segments (1, 2, or 3 depending
    /// on whether the chunk is at the start/end or in the middle).
    ///
    /// # Arguments
    /// * `style` - The `Style` to apply to the `chunk`.
    /// * `chunk` - The specific substring within the paragraph's text to apply the style to. Must not be empty.
    ///
    /// # Errors
    /// * `ParagraphModifyError::ChunkNotFound` - If the `chunk` is not found within any single `StyledText` segment.
    /// * `ParagraphModifyError::EmptyChunk` - If the provided `chunk` is empty.
    ///
    /// # Limitations
    /// * Only modifies the *first* occurrence of the `chunk`.
    /// * Does not handle cases where the `chunk` might span across multiple `StyledText` segments.
    // TODO: Re-evaluate this implementation for efficiency and potentially support chunk spanning multiple segments.
    pub fn modify(&mut self, style: Style, chunk: &str) -> Result<(), ParagraphModifyError> {
        if chunk.is_empty() {
            return Err(ParagraphModifyError::EmptyChunk);
        }

        let found = self
            .raw
            .iter()
            .enumerate()
            .find(|(_n, st)| st.text.contains(chunk))
            .map(|(n, st)| (n, st.clone()));

        let (idx, original_st) =
            found.ok_or_else(|| ParagraphModifyError::ChunkNotFound(chunk.to_string()))?;

        let start_offset = original_st
            .text
            .find(chunk)
            // This unwrap is safe because we already know the chunk is contained via the iterator find above.
            .unwrap();
        let end_offset = start_offset + chunk.len();

        let prefix_text = &original_st.text[..start_offset];
        let suffix_text = &original_st.text[end_offset..];

        let mut replacements = Vec::with_capacity(3);

        if !prefix_text.is_empty() {
            replacements.push(StyledText::new(
                prefix_text.to_string(),
                original_st.style.clone(),
            ));
        }

        replacements.push(StyledText::new(chunk.to_string(), style));

        if !suffix_text.is_empty() {
            replacements.push(StyledText::new(suffix_text.to_string(), original_st.style));
        }

        self.raw.splice(idx..=idx, replacements);

        Ok(())
    }

    /// Renders the paragraph as a single string with inline style tags.
    /// Used primarily for debugging or simple text representations.
    /// The exact tag format depends on the `Display` implementation of `Style`.
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
    fn test_paragraph_new_and_default() {
        let p_new = StyledParagraph::new();
        let p_default = StyledParagraph::default();
        assert!(p_new.raw.is_empty());
        assert_eq!(p_new, p_default); // Check new is same as default
    }

    #[test]
    fn test_paragraph_add() {
        let mut p = StyledParagraph::new();
        let st1 = StyledText::new("Hello ".to_string(), Style::new());
        let st2 = StyledText::new("World".to_string(), Style::new().switch_bold());
        p.add(st1.clone()); // Clone st1 for later comparison if needed
        p.add(st2.clone()); // Clone st2 for later comparison if needed
        assert_eq!(p.raw.len(), 2);
        assert_eq!(p.raw[0], st1);
        assert_eq!(p.raw[1], st2);
    }

    #[test]
    fn test_paragraph_insert() {
        let mut p = StyledParagraph::new();
        let st1 = StyledText::new("First".to_string(), Style::new());
        let st2 = StyledText::new("Third".to_string(), Style::new());
        let st_ins = StyledText::new("Second".to_string(), Style::new().switch_italic());
        p.add(st1.clone());
        p.add(st2.clone());
        p.insert(1, st_ins.clone()); // Insert at index 1

        assert_eq!(p.raw.len(), 3);
        assert_eq!(p.raw[0], st1);
        assert_eq!(p.raw[1], st_ins);
        assert_eq!(p.raw[2], st2);
    }

    #[test]
    #[should_panic]
    fn test_paragraph_insert_out_of_bounds() {
        let mut p = StyledParagraph::new();
        let st1 = StyledText::new("First".to_string(), Style::new());
        p.insert(1, st1); // Panics because index 1 is out of bounds for empty vec
    }

    #[test]
    fn test_paragraph_modify_simple() {
        let mut p = StyledParagraph::new();
        let original_style = Style::new();
        let st1 = StyledText::new("This is a test.".to_string(), original_style.clone());
        p.add(st1);

        let bold_style = Style::new().switch_bold();
        let result = p.modify(bold_style.clone(), "is a");

        assert!(result.is_ok());
        assert_eq!(p.raw.len(), 3);
        // Check prefix
        assert_eq!(p.raw[0].text, "This ");
        assert_eq!(p.raw[0].style, original_style);
        // Check modified chunk
        assert_eq!(p.raw[1].text, "is a");
        assert_eq!(p.raw[1].style, bold_style);
        // Check suffix
        assert_eq!(p.raw[2].text, " test.");
        assert_eq!(p.raw[2].style, original_style);
    }

    #[test]
    fn test_paragraph_modify_full_chunk_match() {
        // Tests modifying a chunk that exactly matches an existing StyledText
        let mut p = StyledParagraph::new();
        let original_style = Style::new();
        let st1 = StyledText::new("Part1 ".to_string(), original_style.clone());
        let st2 = StyledText::new("ModifyMe".to_string(), original_style.clone());
        let st3 = StyledText::new(" Part3".to_string(), original_style.clone());
        p.add(st1.clone());
        p.add(st2); // No clone needed as it will be replaced
        p.add(st3.clone());

        let italic_style = Style::new().switch_italic();
        let result = p.modify(italic_style.clone(), "ModifyMe");

        assert!(result.is_ok());
        // Should replace st2, resulting in 3 chunks total
        assert_eq!(p.raw.len(), 3);
        assert_eq!(p.raw[0], st1); // Check prefix chunk
        // Check modified chunk (index 1)
        assert_eq!(p.raw[1].text, "ModifyMe");
        assert_eq!(p.raw[1].style, italic_style);
        assert_eq!(p.raw[2], st3); // Check suffix chunk
    }

    #[test]
    fn test_paragraph_modify_start_of_chunk() {
        let mut p = StyledParagraph::new();
        let original_style = Style::new();
        let st1 = StyledText::new("Prefix suffix".to_string(), original_style.clone());
        p.add(st1);

        let bold_style = Style::new().switch_bold();
        let result = p.modify(bold_style.clone(), "Prefix");

        assert!(result.is_ok());
        // Should split into 2 chunks
        assert_eq!(p.raw.len(), 2);
        // Check modified chunk (index 0)
        assert_eq!(p.raw[0].text, "Prefix");
        assert_eq!(p.raw[0].style, bold_style);
        // Check suffix chunk (index 1)
        assert_eq!(p.raw[1].text, " suffix");
        assert_eq!(p.raw[1].style, original_style);
    }

    #[test]
    fn test_paragraph_modify_end_of_chunk() {
        let mut p = StyledParagraph::new();
        let original_style = Style::new();
        let st1 = StyledText::new("Prefix suffix".to_string(), original_style.clone());
        p.add(st1);

        let bold_style = Style::new().switch_bold();
        let result = p.modify(bold_style.clone(), "suffix");

        assert!(result.is_ok());
        // Should split into 2 chunks
        assert_eq!(p.raw.len(), 2);
        // Check prefix chunk (index 0)
        assert_eq!(p.raw[0].text, "Prefix ");
        assert_eq!(p.raw[0].style, original_style);
        // Check modified chunk (index 1)
        assert_eq!(p.raw[1].text, "suffix");
        assert_eq!(p.raw[1].style, bold_style);
    }

    #[test]
    fn test_paragraph_modify_chunk_not_found() {
        let mut p = StyledParagraph::new();
        let original_style = Style::new();
        let st1 = StyledText::new("Some text here.".to_string(), original_style.clone());
        p.add(st1.clone());

        let bold_style = Style::new().switch_bold();
        let result = p.modify(bold_style, "nonexistent");

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            ParagraphModifyError::ChunkNotFound("nonexistent".to_string())
        );
        // Ensure original state is preserved
        assert_eq!(p.raw.len(), 1);
        assert_eq!(p.raw[0], st1);
    }

    #[test]
    fn test_paragraph_modify_empty_chunk() {
        let mut p = StyledParagraph::new();
        let original_style = Style::new();
        let st1 = StyledText::new("Some text".to_string(), original_style.clone());
        p.add(st1.clone());

        let bold_style = Style::new().switch_bold();
        let result = p.modify(bold_style, ""); // Empty chunk

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ParagraphModifyError::EmptyChunk);
        // Ensure original state is preserved
        assert_eq!(p.raw.len(), 1);
        assert_eq!(p.raw[0], st1);
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

    #[test]
    fn test_parse_as_raw_tagged_text_empty() {
        let p = StyledParagraph::new();
        assert_eq!(p.parse_as_raw_tagged_text(), "");
    }
}
