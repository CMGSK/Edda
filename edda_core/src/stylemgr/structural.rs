use std::collections::VecDeque;

use super::{style::Style, text::StyledText};

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
    pub raw: Vec<StyledText>,
}

impl StyledParagraph {
    fn new() -> Self {
        StyledParagraph {
            raw: Vec::new().into(),
        }
    }

    fn insert(&mut self, idx: usize, new: StyledText) {
        self.raw.insert(idx, new);
    }

    fn add(&mut self, new: StyledText) {
        self.raw.push(new);
    }

    //TODO: This is hideous
    fn modify(&mut self, style: Style, chunk: &str) {
        let (mut idx, dif) = self
            .raw
            .iter()
            .enumerate()
            .find(|(_n, st)| st.text.contains(chunk))
            .unwrap();

        let prepend = StyledText::new(
            dif.text[..dif.text.find(chunk).unwrap_or(0)].into(),
            dif.style.clone(),
        );
        let append = StyledText::new(
            dif.text[(dif.text.find(chunk).unwrap_or(0) + chunk.len())..].into(),
            dif.style.clone(),
        );
        let new_st = StyledText::new(chunk.into(), style);

        self.raw.remove(idx);
        if !prepend.text.is_empty() {
            self.raw.insert(idx, prepend);
            idx += 1;
        }
        self.raw.insert(idx, new_st);
        idx += 1;
        if !append.text.is_empty() {
            self.raw.insert(idx, append);
        }
    }

    fn parse_as_raw_tagged_text(self) -> String {
        self.raw
            .iter()
            .map(|x| x.clone().apply_style_tagging())
            .fold(String::new(), |mut acc, s| {
                acc.push_str(&s);
                acc
            })
    }
}
