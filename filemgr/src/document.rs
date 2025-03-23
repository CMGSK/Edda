use std::{fs::File, io};

use docx_rs::{Docx, Paragraph, Run};
use ropey::Rope;

pub struct Document {
    content: Rope,
    metadata: Metadata,
}

pub struct Metadata {
    title: String,
}

impl Document {
    /// Create a blank document
    pub fn new(title: &str) -> Self {
        Self {
            content: Rope::new(),
            metadata: Metadata {
                title: title.into(),
            },
        }
    }

    /// Get full document as string
    pub fn get_text(&self) -> String {
        self.content.to_string()
    }

    pub fn save_as_docx(&self, path: &str) -> io::Result<()> {
        let mut document = Docx::new();

        // A Paragraph is a block of text
        // A Run is a segment of text inside paragraphs to distinguish styling
        // TODO: This DOES NOT allow us to style isolated fragments within a line.
        for line in self.get_text().lines() {
            document = document.add_paragraph(Paragraph::new().add_run(Run::new().add_text(line)));
        }

        let mut file = File::create(path)?;
        document.build().pack(&mut file)?;

        Ok(())
    }
}
