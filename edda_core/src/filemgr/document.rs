use std::fmt::Write;
use std::path::Path;
use std::{fs::File, io};

use docx_rs::{Docx, Paragraph};

use crate::stylemgr::structural::StyledParagraph;
#[allow(unused_imports)]
use crate::stylemgr::style::Style;
#[allow(unused_imports)]
use crate::stylemgr::text::StyledText;

pub struct Document {
    content: Vec<StyledParagraph>,
    metadata: Metadata,
}

#[allow(dead_code)]
#[derive(Default, Debug)]
pub struct Metadata {
    title: String,
    authors: Option<Vec<String>>,
    description: Option<String>,
    category: Option<String>,
    version: Option<String>,
    status: Option<String>,
    language: Option<String>,
    keywords: Option<Vec<String>>,
}

impl Document {
    /// Create a blank document
    pub fn new(title: &str) -> Self {
        Self {
            content: Vec::new(),
            metadata: Metadata {
                title: title.into(),
                ..Default::default()
            },
        }
    }

    pub fn get_metadata(&self) -> &Metadata {
        &self.metadata
    }
    /// Get full document as string
    pub fn get_text(&self, tagged: bool) -> String {
        let mut buffer = String::with_capacity(self.content.len() * 100);

        for sp in &self.content {
            for x in &sp.raw {
                if tagged {
                    let _ = write!(buffer, "{}", x.clone().apply_style_tagging());
                } else {
                    buffer.push_str(&x.text);
                }
            }
        }
        buffer
    }

    pub fn save_as_docx<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let mut document = Docx::new();

        for styled_paragraph in &self.content {
            let mut docx_paragraph = Paragraph::new();

            for styled_text in &styled_paragraph.raw {
                let run = styled_text.apply_to_raw();
                docx_paragraph = docx_paragraph.add_run(run);
            }

            document = document.add_paragraph(docx_paragraph);
        }

        let mut file = File::create(path)?;
        document.build().pack(&mut file)?;

        Ok(())
    }
}
