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
#[derive(Debug)]
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

    }
}
