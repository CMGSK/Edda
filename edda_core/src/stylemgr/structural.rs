use std::collections::VecDeque;

use super::text::StyledText;

pub enum ApplicableStyles {
    Bold,
    Italic,
    Underline,
    Size(u8),
    Font(String),
    Color(String),
    Highlight(Option<String>),
}

/// Collection of text chunks with its own styles
pub struct StyledParagraph {
    pub raw: VecDeque<StyledText>,
}

impl StyledParagraph {
    fn new() -> Self {
        StyledParagraph {
            raw: Vec::new().into(),
        }
    }

    fn prepend(&mut self, new: StyledText) {
        self.raw.push_front(new);
    }
    
    fn add(&mut self, new: StyledText) {
        self.raw.push_back(new);
    }

    fn parse_raw_tagged_text(text: &str) -> Self {
        todo!()
    }
}
